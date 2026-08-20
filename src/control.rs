use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlMode {
    KeyboardMouse,
    Touch,
}

pub struct InputState {
    pub move_vec: Vec2,
    pub aim_dir: Vec2,
    pub attack_triggered: bool,
    pub is_shooting: bool,
    pub pickup_triggered: bool,
}

pub struct ControlsManager {
    pub mode: ControlMode,

    // UI стика и кнопок
    stick_center: Vec2,
    stick_radius: f32,
    stick_handle: Vec2,
    touch_move_dir: Vec2,

    btn_shoot_center: Vec2,
    btn_shoot_radius: f32,
    is_shooting_touch: bool,

    btn_pickup_center: Vec2,
    btn_pickup_radius: f32,
    pickup_pressed_touch: bool,
}

impl ControlsManager {
    pub fn new() -> Self {
        // На Android по умолчанию Touch, на PC — Клавиатура
        let mode = if cfg!(target_os = "android") {
            ControlMode::Touch
        } else {
            ControlMode::KeyboardMouse
        };

        Self {
            mode,
            stick_center: Vec2::ZERO,
            stick_radius: 80.0,
            stick_handle: Vec2::ZERO,
            touch_move_dir: Vec2::ZERO,

            btn_shoot_center: Vec2::ZERO,
            btn_shoot_radius: 50.0,
            is_shooting_touch: false,

            btn_pickup_center: Vec2::ZERO,
            btn_pickup_radius: 40.0,
            pickup_pressed_touch: false,
        }
    }

    fn update_ui_positions(&mut self) {
        let h = screen_height();
        let w = screen_width();
        self.stick_center = vec2(120.0, h - 120.0);
        self.btn_shoot_center = vec2(w - 100.0, h - 100.0);
        self.btn_pickup_center = vec2(w - 100.0, h - 220.0);
    }

    pub fn update(&mut self, camera: &Camera2D, player_pos: Vec2) -> InputState {
        self.update_ui_positions();
        let active_touches = touches();

        // Если прикоснулись к экрану — авто-переключаемся на Touch режим
        if !active_touches.is_empty() {
            self.mode = ControlMode::Touch;
        }

        match self.mode {
            ControlMode::Touch => self.update_touch(active_touches),
            ControlMode::KeyboardMouse => self.update_keyboard_mouse(camera, player_pos),
        }
    }

    fn update_touch(&mut self, active_touches: Vec<Touch>) -> InputState {
        self.touch_move_dir = Vec2::ZERO;
        self.stick_handle = self.stick_center;
        self.is_shooting_touch = false;
        self.pickup_pressed_touch = false;

        for touch in active_touches {
            let pos = touch.position;

            // Стик
            if pos.distance(self.stick_center) <= self.stick_radius * 2.0 {
                let delta = pos - self.stick_center;
                let dist = delta.length();
                let clamped_dist = dist.min(self.stick_radius);

                self.touch_move_dir = delta.normalize_or_zero();
                self.stick_handle = self.stick_center + self.touch_move_dir * clamped_dist;
            }

            // Кнопка стрельбы
            if pos.distance(self.btn_shoot_center) <= self.btn_shoot_radius {
                self.is_shooting_touch = true;
            }

            // Кнопка подбора
            if pos.distance(self.btn_pickup_center) <= self.btn_pickup_radius {
                if touch.phase == TouchPhase::Started {
                    self.pickup_pressed_touch = true;
                }
            }
        }

        InputState {
            move_vec: self.touch_move_dir,
            aim_dir: self.touch_move_dir, // Целимся в сторону движения стика
            attack_triggered: self.is_shooting_touch,
            is_shooting: self.is_shooting_touch,
            pickup_triggered: self.pickup_pressed_touch,
        }
    }

    fn update_keyboard_mouse(&mut self, camera: &Camera2D, player_pos: Vec2) -> InputState {
        let mut move_vec = Vec2::ZERO;
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

        let mouse_screen = mouse_position();
        let mouse_world = camera.screen_to_world(vec2(mouse_screen.0, mouse_screen.1));
        let aim_dir = (mouse_world - player_pos).normalize_or_zero();

        InputState {
            move_vec: move_vec.normalize_or_zero(),
            aim_dir,
            attack_triggered: is_mouse_button_pressed(MouseButton::Left),
            is_shooting: is_mouse_button_down(MouseButton::Left),
            pickup_triggered: is_mouse_button_pressed(MouseButton::Right),
        }
    }

    pub fn draw(&self) {
        if self.mode != ControlMode::Touch {
            return;
        }

        set_default_camera();

        // Стик
        draw_circle(
            self.stick_center.x,
            self.stick_center.y,
            self.stick_radius,
            Color::new(1.0, 1.0, 1.0, 0.2),
        );
        draw_circle(
            self.stick_handle.x,
            self.stick_handle.y,
            35.0,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );

        // Выстрел
        let shoot_color = if self.is_shooting_touch {
            RED
        } else {
            Color::new(1.0, 0.0, 0.0, 0.4)
        };
        draw_circle(
            self.btn_shoot_center.x,
            self.btn_shoot_center.y,
            self.btn_shoot_radius,
            shoot_color,
        );

        // Подбор
        let pickup_color = if self.pickup_pressed_touch {
            GREEN
        } else {
            Color::new(0.0, 1.0, 0.0, 0.4)
        };
        draw_circle(
            self.btn_pickup_center.x,
            self.btn_pickup_center.y,
            self.btn_pickup_radius,
            pickup_color,
        );
    }
}
