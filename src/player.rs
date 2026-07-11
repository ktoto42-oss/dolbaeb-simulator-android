use macroquad::prelude::*;
use crate::GameState;
use crate::{STREET_WIDTH, STREET_HEIGHT, APT_HEIGHT, APT_WIDTH};
use crate::assets::Assets;

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

// Пистолет
const PISTOL_ROW: usize = 2;
const PISTOL_FRAMES: usize = 2;

// Автомат
const RIFLE_ROW: usize = 3;
const RIFLE_FRAMES: usize = 2;

// Ноги
const LEGS_ROW: usize = 4;
const LEGS_FRAMES: usize = 7;

// Всё оружие
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weapon {
    Fists,
    Pipe,
    Pistol,
    Rifle,
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

    //
    pub fn update(&mut self, dt: f32) -> bool {
        if self.num_frames <= 1 { return false; }

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

    // Вспомогательный метод для быстрой смены параметров анимации
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

    // Новые поля для контроля состояний боевой системы
    pub current_weapon: Weapon,
    pub is_attacking: bool,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32) -> Self {
        Self {
            x,
            y,
            speed,
            rotation: 0.0,
            legs_rotation: 0.0,
            // Изначально торс стоит в режиме "Кулаки" (1 кадр в строке PUNCH_ROW)
            torso_anim: AnimationState::new(1, 1.0, PUNCH_ROW),
            legs_anim: AnimationState::new(LEGS_FRAMES, 14.0, LEGS_ROW),
            is_moving: false,
            is_attacking: false,
            current_weapon: Weapon::Fists,
        }
    }

    // Управление
    pub fn handle_input(&mut self, delta_time: f32) {
        // Выбор оружия
        if !self.is_attacking {
            let old_weapon = self.current_weapon;

            if is_key_pressed(KeyCode::Key1) { self.current_weapon = Weapon::Fists; }
            if is_key_pressed(KeyCode::Key2) { self.current_weapon = Weapon::Pipe; }
            if is_key_pressed(KeyCode::Key3) { self.current_weapon = Weapon::Pistol; }
            if is_key_pressed(KeyCode::Key4) { self.current_weapon = Weapon::Rifle; }

            // При смене оружия ставит торс в состояние покоя
            if self.current_weapon != old_weapon {
                let row = match self.current_weapon {
                    Weapon::Fists => PUNCH_ROW,
                    Weapon::Pipe => PIPE_ROW,
                    Weapon::Pistol => PISTOL_ROW,
                    Weapon::Rifle => RIFLE_ROW,
                };
                self.torso_anim.set_state(row, 1, 1.0);
            }
        }

        // Движение
        let mut move_vec = vec2(0.0, 0.0);
        if is_key_down(KeyCode::W) { move_vec.y -= 1.0; }
        if is_key_down(KeyCode::S) { move_vec.y += 1.0; }
        if is_key_down(KeyCode::A) { move_vec.x -= 1.0; }
        if is_key_down(KeyCode::D) { move_vec.x += 1.0; }

        self.is_moving = move_vec != vec2(0.0, 0.0);

        if self.is_moving {
            move_vec = move_vec.normalize();
            self.x += move_vec.x * self.speed * delta_time;
            self.y += move_vec.y * self.speed * delta_time;

            self.legs_rotation = move_vec.y.atan2(move_vec.x);
            self.legs_anim.update(delta_time);
        } else {
            self.legs_anim.reset();
        }

        // Атака
        let attack_triggered = match self.current_weapon {
            Weapon::Rifle => is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::E),
            _ => is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::E),
        };

        if attack_triggered && !self.is_attacking {
            self.is_attacking = true;

            let (row, frames, fps) = match self.current_weapon {
                Weapon::Fists => (PUNCH_ROW, PUNCH_FRAMES, 14.0),
                Weapon::Pipe => (PIPE_ROW, PIPE_FRAMES, 14.0),
                Weapon::Pistol => (PISTOL_ROW, PISTOL_FRAMES, 12.0),
                Weapon::Rifle => (RIFLE_ROW, RIFLE_FRAMES, 14.0),
            };
            self.torso_anim.set_state(row, frames, fps);
        }

        // Обновление анимации торса
        if self.is_attacking {
            let animation_finished = self.torso_anim.update(delta_time);

            if animation_finished {
                // Если это автомат и кнопка всё ещё зажата перезапуск анимации
                let keep_shooting = self.current_weapon == Weapon::Rifle &&
                    (is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::E));

                if keep_shooting {
                    self.torso_anim.reset();
                } else {
                    self.is_attacking = false;
                    let row = match self.current_weapon {
                        Weapon::Fists => PUNCH_ROW,
                        Weapon::Pipe => PIPE_ROW,
                        Weapon::Pistol => PISTOL_ROW,
                        Weapon::Rifle => RIFLE_ROW,
                    };
                    self.torso_anim.set_state(row, 1, 1.0);
                }
            }
        }
    }

    // Обновление вращения торса за мышью
    pub fn update_rotation(&mut self, camera: &Camera2D) {
        let mouse_screen = mouse_position();
        let mouse_world = camera.screen_to_world(vec2(mouse_screen.0, mouse_screen.1));
        let direction = mouse_world - vec2(self.x, self.y);

        self.rotation = direction.y.atan2(direction.x);
    }

    // Ограничение локаций
    pub fn location_restriction(&mut self, state: &GameState) {
        match state {
            GameState::InApartment => {
                self.x = self.x.clamp(0.0, APT_WIDTH);
                self.y = self.y.clamp(0.0, APT_HEIGHT);
            }
            GameState::OnStreet => {
                self.x = self.x.clamp(0.0, STREET_WIDTH);
                self.y = self.y.clamp(0.0, STREET_HEIGHT);
            }
            _ => {}
        }
    }

    // Отрисовка игрока
    pub fn draw(&self, assets: &Assets) {
        let half_scaled_size = SCALED_SIZE / 2.0;
        let visual_offset_x = SPRITE_OFFSET_X * SCALE;
        let visual_offset_y = SPRITE_OFFSET_Y * SCALE;

        // Отрисовка ног
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
                source: Some(Rect::new(torso_src_x, torso_src_y, SPRITE_SIZE, SPRITE_SIZE)),
                rotation: self.rotation,
                pivot: Some(vec2(self.x, self.y)),
                ..Default::default()
            },
        );
    }
}
