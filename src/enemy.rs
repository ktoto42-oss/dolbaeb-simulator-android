use crate::assets::Assets;
use crate::objects::{Bullet, DroppedWeapon};
use crate::player::{AnimationState, Player, Weapon};
use crate::player::{
    LEGS_FRAMES, LEGS_ROW, PUNCH_ROW, SCALE, SCALED_SIZE, SPRITE_OFFSET_X, SPRITE_OFFSET_Y,
    SPRITE_SIZE,
};
use crate::tilemap::{TilemapManager, WorldManager};
use macroquad::audio::play_sound_once;
use macroquad::prelude::*;

// Состояния
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyState {
    Patrol,
    MeleeChase,
    RangedChase,
}

// Структура врага
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub rotation: f32,
    pub torso_anim: AnimationState,
    pub legs_anim: AnimationState,
    pub weapon: Weapon,
    pub is_dead: bool,
    pub is_moving: bool,
    pub is_attacking: bool,
    pub state: EnemyState,
    pub patrol_dir: Vec2,
    pub shoot_cooldown: f32,
    pub bullets: Vec<Bullet>,
}

impl Enemy {
    pub fn new(pos: Vec2, weapon: Weapon, patrol_dir: Vec2) -> Self {
        let dir = patrol_dir.normalize_or_zero();
        let initial_rot = if dir != Vec2::ZERO {
            dir.y.atan2(dir.x)
        } else {
            0.0
        };

        Self {
            x: pos.x,
            y: pos.y,
            speed: 180.0,
            rotation: initial_rot,
            torso_anim: AnimationState::new(1, 1.0, PUNCH_ROW),
            legs_anim: AnimationState::new(LEGS_FRAMES, 14.0, LEGS_ROW),
            weapon,
            is_dead: false,
            is_moving: false,
            is_attacking: false,
            state: EnemyState::Patrol,
            patrol_dir: dir,
            shoot_cooldown: 0.0,
            bullets: Vec::new(),
        }
    }

    pub fn collider(&self) -> Rect {
        let width = 36.0;
        let height = 36.0;

        Rect::new(self.x - width / 2.0, self.y, width, height)
    }

    pub fn update(
        &mut self,
        player: &mut Player,
        world_manager: &WorldManager,
        dropped_weapons: &mut Vec<DroppedWeapon>,
        dt: f32,
        assets: &Assets,
    ) {
        let active_map = world_manager.get_active();
        let player_rect = player.collider();
        let map_bounds = active_map.bounds();

        self.bullets.retain_mut(|bullet| {
            bullet.update(dt);

            if bullet.lifetime <= 0.0 || active_map.check_collision(bullet.collider()) {
                return false;
            }

            if !player.is_dead && bullet.collider().overlaps(&player_rect) {
                player.is_dead = true;
                return false;
            }

            true
        });

        if self.is_dead {
            return;
        }

        let enemy_rect = self.collider();
        let mut hit_by_player_bullet = false;
        let enemy_pos = vec2(self.x, self.y);
        let player_pos = vec2(player.x, player.y);

        player.bullets.retain(|b| {
            if !hit_by_player_bullet && b.collider().overlaps(&enemy_rect) {
                hit_by_player_bullet = true;
                false
            } else {
                true
            }
        });

        if hit_by_player_bullet {
            self.die(dropped_weapons);
            return;
        }

        let dist_to_player = vec2(self.x, self.y).distance(vec2(player.x, player.y));
        let is_player_melee = player.weapon != Weapon::Pistol
            && player.weapon != Weapon::Rifle
            && player.weapon != Weapon::Dead;

        if player.is_attacking && is_player_melee && dist_to_player < player.atack_radius {
            match player.weapon {
                Weapon::Knife => play_sound_once(&assets.sound_knife),
                Weapon::Pipe | Weapon::Fists => play_sound_once(&assets.sound_pipe),
                _ => {}
            }
            self.die(dropped_weapons);
            return;
        }

        let sees_player = (dist_to_player < 200.0
            || (dist_to_player < 400.0 && active_map.has_line_of_sight(enemy_pos, player_pos)))
            && !player.is_dead;

        if sees_player {
            self.state = match self.weapon {
                Weapon::Pistol | Weapon::Rifle => EnemyState::RangedChase,
                _ => EnemyState::MeleeChase,
            };
        } else {
            self.state = EnemyState::Patrol;
        }

        match self.state {
            EnemyState::Patrol => {
                if self.patrol_dir != Vec2::ZERO {
                    let move_vec = self.patrol_dir * self.speed * dt;
                    let old_x = self.x;
                    let old_y = self.y;

                    self.x += move_vec.x;
                    self.y += move_vec.y;
                    self.is_moving = true;

                    let col = self.collider();

                    let hit_obstacle = active_map.check_collision(col)
                        || col.x < 0.0
                        || col.x + col.w > map_bounds.w
                        || col.y < 0.0
                        || col.y + col.h > map_bounds.h;

                    if hit_obstacle {
                        self.x = old_x;
                        self.y = old_y;
                        self.patrol_dir = -self.patrol_dir;
                    }

                    self.rotation = self.patrol_dir.y.atan2(self.patrol_dir.x);
                }
            }

            EnemyState::MeleeChase => {
                self.is_moving = true;

                let dir = active_map.get_flow_direction(enemy_pos);

                if dir != Vec2::ZERO {
                    let old_x = self.x;
                    self.x += dir.x * self.speed * dt;
                    if active_map.check_collision(self.collider()) {
                        self.x = old_x;
                    }

                    let old_y = self.y;
                    self.y += dir.y * self.speed * dt;
                    if active_map.check_collision(self.collider()) {
                        self.y = old_y;
                    }

                    self.rotation = dir.y.atan2(dir.x);
                } else {
                    let to_player = (player_pos - enemy_pos).normalize_or_zero();
                    self.rotation = to_player.y.atan2(to_player.x);
                }

                if dist_to_player < 80.0 {
                    if !self.is_attacking {
                        self.is_attacking = true;
                        play_sound_once(&assets.sound_swosh);
                        match self.weapon {
                            Weapon::Knife => play_sound_once(&assets.sound_knife),
                            Weapon::Pipe | Weapon::Fists => play_sound_once(&assets.sound_pipe),
                            _ => {}
                        }
                        let (row, frames, fps) = self.weapon.anim_info();
                        self.torso_anim.set_state(row, frames, fps);
                    }
                    player.is_dead = true;
                }
            }

            EnemyState::RangedChase => {
                let dir_to_player = (player_pos - enemy_pos).normalize_or_zero();
                self.rotation = dir_to_player.y.atan2(dir_to_player.x);

                let move_dir = active_map.get_flow_direction(enemy_pos);

                if dist_to_player > 180.0 {
                    self._move(move_dir * self.speed * dt, active_map);
                }

                if self.shoot_cooldown <= 0.0 {
                    self.is_attacking = true;
                    match self.weapon {
                        Weapon::Rifle => play_sound_once(&assets.sound_ak47),
                        Weapon::Pistol => play_sound_once(&assets.sound_pistol),
                        _ => {}
                    }
                    self.bullets.push(Bullet::new(enemy_pos, dir_to_player));
                    self.shoot_cooldown = match self.weapon {
                        Weapon::Rifle => 0.18,
                        _ => 0.6,
                    };
                }
            }
        }

        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= dt;
        }

        if !self.is_attacking {
            let (row, frames, fps) = self.weapon.anim_info();
            self.torso_anim.set_state(row, frames, fps);
        }

        // Обновление анимации торса
        if self.is_attacking {
            let animation_finished = self.torso_anim.update(dt);

            if animation_finished {
                // Если это автомат и кнопка всё ещё зажата перезапуск анимации
                let keep_shooting = self.weapon == Weapon::Rifle
                    && (is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::E));

                if keep_shooting {
                    self.torso_anim.reset();
                } else {
                    self.is_attacking = false;
                    let row = self.weapon.anim_info().0;
                    self.torso_anim.set_state(row, 1, 1.0);
                }
            }
        }
        if self.is_moving {
            self.legs_anim.update(dt);
        } else {
            self.legs_anim.reset();
        }
    }

    fn _move(&mut self, move_vec: Vec2, active_map: &TilemapManager) {
        let old_x = self.x;
        let old_y = self.y;

        self.x += move_vec.x;
        if active_map.check_collision(self.collider()) {
            self.x = old_x;
        }

        self.y += move_vec.y;
        if active_map.check_collision(self.collider()) {
            self.y = old_y;
        }
    }

    // Смерть врага с дропом его оружия
    pub fn die(&mut self, dropped_weapons: &mut Vec<DroppedWeapon>) {
        if self.is_dead {
            return;
        }
        self.is_dead = true;

        if self.weapon != Weapon::Fists && self.weapon != Weapon::Dead {
            let ammo = match self.weapon {
                Weapon::Pistol => 12,
                Weapon::Rifle => 30,
                _ => 0,
            };

            dropped_weapons.push(DroppedWeapon::new(
                vec2(self.x, self.y),
                self.weapon,
                ammo,
                self.rotation,
            ));
        }

        self.weapon = Weapon::Dead;

        let (row, frames, fps) = self.weapon.anim_info();
        self.torso_anim.set_state(row, frames, fps);
    }

    pub fn draw(&self, assets: &Assets) {
        for bullet in &self.bullets {
            bullet.draw();
        }

        let half_scaled_size = SCALED_SIZE / 2.0;
        let visual_offset_x = SPRITE_OFFSET_X * SCALE;
        let visual_offset_y = SPRITE_OFFSET_Y * SCALE;

        // Отрисовка ног
        if !self.is_dead {
            let legs_src_x = self.legs_anim.current_frame as f32 * SPRITE_SIZE;
            let legs_src_y = self.legs_anim.row_index * SPRITE_SIZE;

            draw_texture_ex(
                &assets.enemy,
                self.x - half_scaled_size,
                self.y - half_scaled_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SCALED_SIZE, SCALED_SIZE)),
                    source: Some(Rect::new(legs_src_x, legs_src_y, SPRITE_SIZE, SPRITE_SIZE)),
                    rotation: self.rotation,
                    pivot: Some(vec2(self.x, self.y)),
                    ..Default::default()
                },
            );
        }

        // Отрисовка торса
        let torso_src_x = self.torso_anim.current_frame as f32 * SPRITE_SIZE;
        let torso_src_y = self.torso_anim.row_index * SPRITE_SIZE;

        draw_texture_ex(
            &assets.enemy,
            self.x - half_scaled_size + visual_offset_x,
            self.y - half_scaled_size + visual_offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SCALED_SIZE, SCALED_SIZE)),
                source: Some(Rect::new(
                    torso_src_x,
                    torso_src_y,
                    SPRITE_SIZE,
                    SPRITE_SIZE,
                )),
                rotation: self.rotation,
                pivot: Some(vec2(self.x, self.y)),
                ..Default::default()
            },
        );
    }
}
