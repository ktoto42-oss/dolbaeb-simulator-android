use crate::assets::Assets;
use crate::npc::Enemy;
use crate::objects::{Bullet, DroppedWeapon};
use crate::tilemap::{TilemapManager, WorldManager};
use macroquad::prelude::*;

// Настройка спрайтов и смещения центра
const SPRITE_SIZE: f32 = 48.0;
const SCALE: f32 = 3.0;
const SCALED_SIZE: f32 = SPRITE_SIZE * SCALE;

// Сдвиг текстур (тайлсет кривой что пиздец)
const SPRITE_OFFSET_X: f32 = 4.0;
const SPRITE_OFFSET_Y: f32 = 0.0;

// Константы строк и кадров для тайлсета
// Кулаки
const PUNCH_ROW: usize = 0;
const PUNCH_FRAMES: usize = 7;

// Труба
const PIPE_ROW: usize = 1;
const PIPE_FRAMES: usize = 7;

// Нож
const KNIGHT_ROW: usize = 2;
const KNIGHT_FRAMES: usize = 5;

// Пистолет
const PISTOL_ROW: usize = 3;
const PISTOL_FRAMES: usize = 2;

// Автомат
const RIFLE_ROW: usize = 4;
const RIFLE_FRAMES: usize = 2;

// Ноги
const LEGS_ROW: usize = 5;
const LEGS_FRAMES: usize = 7;

const DEAD_ROW: usize = 6;
const DEAD_FRAMES: usize = 1;

// Всё оружие
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weapon {
    Fists,
    Pipe,
    Knight,
    Pistol,
    Rifle,
    Dead,
}

// Структура анимаций
pub struct AnimationState {
    pub current_frame: usize,
    frame_timer: f32,
    frame_delay: f32,
    pub num_frames: usize,
    pub row_index: f32,
}

impl AnimationState {
    pub fn new(num_frames: usize, fps: f32, row_index: usize) -> Self {
        Self {
            current_frame: 0,
            frame_timer: 0.0,
            frame_delay: 1.0 / fps,
            num_frames,
            row_index: row_index as f32,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.num_frames <= 1 {
            return false;
        }

        self.frame_timer += dt;
        if self.frame_timer >= self.frame_delay {
            self.frame_timer -= self.frame_delay;
            let next_frame = self.current_frame + 1;

            if next_frame >= self.num_frames {
                self.current_frame = 0;
                return true;
            } else {
                self.current_frame = next_frame;
            }
        }
        false
    }

    pub fn set_state(&mut self, row: usize, num_frames: usize, fps: f32) {
        self.row_index = row as f32;
        self.num_frames = num_frames;
        self.frame_delay = 1.0 / fps;
        self.current_frame = 0;
        self.frame_timer = 0.0;
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.frame_timer = 0.0;
    }
}

// Структура игрока
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub rotation: f32,
    pub legs_rotation: f32,
    pub torso_anim: AnimationState,
    pub legs_anim: AnimationState,
    pub is_moving: bool,
    pub current_weapon: Weapon,
    pub ammo: u32,
    pub is_attacking: bool,
    pub is_dead: bool,
    pub atack_radius: f32,
    pub bullets: Vec<Bullet>,
    pub shoot_cooldown: f32,
    pub fire_rate: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32) -> Self {
        Self {
            x,
            y,
            speed,
            rotation: 0.0,
            legs_rotation: 0.0,
            torso_anim: AnimationState::new(1, 1.0, PUNCH_ROW),
            legs_anim: AnimationState::new(LEGS_FRAMES, 14.0, LEGS_ROW),
            is_moving: false,
            is_attacking: false,
            is_dead: false,
            atack_radius: 80.0,
            current_weapon: Weapon::Fists,
            ammo: 0,
            bullets: Vec::with_capacity(32),
            shoot_cooldown: 0.0,
            fire_rate: 0.15,
        }
    }

    pub fn collider(&self) -> Rect {
        let width = 32.0;
        let height = 24.0;

        Rect::new(self.x - width / 2.0, self.y + 12.0, width, height)
    }

    pub fn handle_input(
        &mut self,
        delta_time: f32,
        world_manager: &WorldManager,
        camera: &Camera2D,
        dropped_weapons: &mut Vec<DroppedWeapon>,
    ) {
        if self.is_dead {
            return;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let player_rect = self.collider();
            if let Some(idx) = dropped_weapons
                .iter()
                .position(|w| w.collider().overlaps(&player_rect))
            {
                let picked = dropped_weapons.remove(idx);

                if self.current_weapon != Weapon::Fists {
                    dropped_weapons.push(DroppedWeapon::new(
                        vec2(self.x, self.y),
                        self.current_weapon,
                        self.ammo,
                        self.rotation,
                    ));
                }

                self.current_weapon = picked.weapon;
                self.ammo = picked.ammo;

                let row = match self.current_weapon {
                    Weapon::Pipe => PIPE_ROW,
                    Weapon::Knight => KNIGHT_ROW,
                    Weapon::Pistol => PISTOL_ROW,
                    Weapon::Rifle => RIFLE_ROW,
                    _ => PUNCH_ROW,
                };
                self.torso_anim.set_state(row, 1, 1.0);
            } else if self.current_weapon != Weapon::Fists {
                dropped_weapons.push(DroppedWeapon::new(
                    vec2(self.x, self.y),
                    self.current_weapon,
                    self.ammo,
                    self.rotation,
                ));

                self.current_weapon = Weapon::Fists;
                self.ammo = 0;
                self.torso_anim.set_state(PUNCH_ROW, 1, 1.0);
            }
        }

        if !self.is_attacking {
            let old_weapon = self.current_weapon;

            if is_key_pressed(KeyCode::Key1) {
                self.current_weapon = Weapon::Fists;
                self.ammo = 0;
            }
            if is_key_pressed(KeyCode::Key2) {
                self.current_weapon = Weapon::Pipe;
                self.ammo = 0;
            }
            if is_key_pressed(KeyCode::Key3) {
                self.current_weapon = Weapon::Knight;
                self.ammo = 0;
            }
            if is_key_pressed(KeyCode::Key4) {
                self.current_weapon = Weapon::Pistol;
                self.ammo = 12;
            }
            if is_key_pressed(KeyCode::Key5) {
                self.current_weapon = Weapon::Rifle;
                self.ammo = 30;
            }

            if self.current_weapon != old_weapon {
                let row = match self.current_weapon {
                    Weapon::Fists => PUNCH_ROW,
                    Weapon::Pipe => PIPE_ROW,
                    Weapon::Knight => KNIGHT_ROW,
                    Weapon::Pistol => PISTOL_ROW,
                    Weapon::Rifle => RIFLE_ROW,
                    Weapon::Dead => DEAD_ROW,
                };
                self.torso_anim.set_state(row, 1, 1.0);
            }
        }

        let mut move_vec = vec2(0.0, 0.0);
        if is_key_down(KeyCode::W) {
            move_vec.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_vec.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_vec.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            move_vec.x += 1.0;
        }

        self.is_moving = move_vec != vec2(0.0, 0.0);

        if self.is_moving {
            move_vec = move_vec.normalize();
            let old_x = self.x;
            let old_y = self.y;
            let active_map = world_manager.get_active();

            self.x += move_vec.x * self.speed * delta_time;
            if active_map.check_collision(self.collider()) {
                self.x = old_x;
            }

            self.y += move_vec.y * self.speed * delta_time;
            if active_map.check_collision(self.collider()) {
                self.y = old_y;
            }

            self.legs_rotation = move_vec.y.atan2(move_vec.x);
            self.legs_anim.update(delta_time);
        } else {
            self.legs_anim.reset();
        }

        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= delta_time;
        }

        let is_firearm =
            self.current_weapon == Weapon::Pistol || self.current_weapon == Weapon::Rifle;
        let can_shoot = !is_firearm || self.ammo > 0;

        let attack_triggered = match self.current_weapon {
            Weapon::Rifle => is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::E),
            _ => is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::E),
        };

        if attack_triggered && !self.is_attacking && can_shoot {
            self.is_attacking = true;

            let (row, frames, fps) = match self.current_weapon {
                Weapon::Fists => (PUNCH_ROW, PUNCH_FRAMES, 14.0),
                Weapon::Pipe => (PIPE_ROW, PIPE_FRAMES, 14.0),
                Weapon::Knight => (KNIGHT_ROW, KNIGHT_FRAMES, 12.0),
                Weapon::Pistol => (PISTOL_ROW, PISTOL_FRAMES, 12.0),
                Weapon::Rifle => (RIFLE_ROW, RIFLE_FRAMES, 14.0),
                Weapon::Dead => (DEAD_ROW, DEAD_FRAMES, 14.0),
            };
            self.torso_anim.set_state(row, frames, fps);
        }

        if self.is_attacking {
            let animation_finished = self.torso_anim.update(delta_time);

            if animation_finished {
                let keep_shooting =
                    is_firearm && self.ammo > 0 && is_mouse_button_down(MouseButton::Left);

                if is_firearm && self.ammo > 0 {
                    let mouse_world =
                        camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
                    let dir = mouse_world - vec2(self.x, self.y);

                    self.bullets.push(Bullet::new(vec2(self.x, self.y), dir));
                    self.shoot_cooldown = self.fire_rate;
                    self.ammo = self.ammo.saturating_sub(1);
                }

                if keep_shooting {
                    self.torso_anim.reset();
                } else {
                    self.is_attacking = false;
                    let row = match self.current_weapon {
                        Weapon::Fists => PUNCH_ROW,
                        Weapon::Pipe => PIPE_ROW,
                        Weapon::Knight => KNIGHT_ROW,
                        Weapon::Pistol => PISTOL_ROW,
                        Weapon::Rifle => RIFLE_ROW,
                        Weapon::Dead => DEAD_ROW,
                    };
                    self.torso_anim.set_state(row, 1, 1.0);
                }
            }
        }
    }

    pub fn update_rotation(&mut self, camera: &Camera2D) {
        if !self.is_dead {
            let mouse_screen = mouse_position();
            let mouse_world = camera.screen_to_world(vec2(mouse_screen.0, mouse_screen.1));
            let direction = mouse_world - vec2(self.x, self.y);

            self.rotation = direction.y.atan2(direction.x);
        }
    }

    // Ограничение локаций
    pub fn location_restriction(&mut self, active_map: &TilemapManager) {
        let bounds = active_map.bounds();
        self.x = self.x.clamp(0.0, bounds.w);
        self.y = self.y.clamp(0.0, bounds.h);
    }

    pub fn restart(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.is_dead = false;
        self.current_weapon = Weapon::Fists;
        let (row, frames, fps) = match self.current_weapon {
            Weapon::Fists => (PUNCH_ROW, PUNCH_FRAMES, 14.0),
            Weapon::Pipe => (PIPE_ROW, PIPE_FRAMES, 14.0),
            Weapon::Knight => (KNIGHT_ROW, KNIGHT_FRAMES, 12.0),
            Weapon::Pistol => (PISTOL_ROW, PISTOL_FRAMES, 12.0),
            Weapon::Rifle => (RIFLE_ROW, RIFLE_FRAMES, 14.0),
            Weapon::Dead => (DEAD_ROW, DEAD_FRAMES, 14.0),
        };
        self.torso_anim.set_state(row, frames, fps);
    }

    // Обновление пуль
    pub fn update_bullets(&mut self, active_map: &TilemapManager, delta_time: f32) {
        self.bullets.retain_mut(|bullet| {
            bullet.update(delta_time);

            if bullet.lifetime <= 0.0 || active_map.check_collision(bullet.collider()) {
                return false;
            }

            true
        });
    }

    // Отрисовка пуль
    pub fn draw_bullets(&self) {
        for bullet in &self.bullets {
            bullet.draw();
        }
    }

    // Отрисовка игрока
    pub fn draw(&mut self, assets: &Assets) {
        if self.is_dead {
            self.current_weapon = Weapon::Dead;

            let (row, frames, fps) = match self.current_weapon {
                Weapon::Fists => (PUNCH_ROW, PUNCH_FRAMES, 14.0),
                Weapon::Pipe => (PIPE_ROW, PIPE_FRAMES, 14.0),
                Weapon::Knight => (KNIGHT_ROW, KNIGHT_FRAMES, 12.0),
                Weapon::Pistol => (PISTOL_ROW, PISTOL_FRAMES, 12.0),
                Weapon::Rifle => (RIFLE_ROW, RIFLE_FRAMES, 14.0),
                Weapon::Dead => (DEAD_ROW, DEAD_FRAMES, 14.0),
            };
            self.torso_anim.set_state(row, frames, fps);
        }

        let half_scaled_size = SCALED_SIZE / 2.0;
        let visual_offset_x = SPRITE_OFFSET_X * SCALE;
        let visual_offset_y = SPRITE_OFFSET_Y * SCALE;

        // Отрисовка ног
        if !self.is_dead {
            let legs_src_x = self.legs_anim.current_frame as f32 * SPRITE_SIZE;
            let legs_src_y = self.legs_anim.row_index * SPRITE_SIZE;

            draw_texture_ex(
                &assets.player,
                self.x - half_scaled_size,
                self.y - half_scaled_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SCALED_SIZE, SCALED_SIZE)),
                    source: Some(Rect::new(legs_src_x, legs_src_y, SPRITE_SIZE, SPRITE_SIZE)),
                    rotation: self.legs_rotation,
                    pivot: Some(vec2(self.x, self.y)),
                    ..Default::default()
                },
            );
        }

        // Отрисовка торса
        let torso_src_x = self.torso_anim.current_frame as f32 * SPRITE_SIZE;
        let torso_src_y = self.torso_anim.row_index * SPRITE_SIZE;

        draw_texture_ex(
            &assets.player,
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
