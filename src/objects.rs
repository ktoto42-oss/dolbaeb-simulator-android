use crate::assets::Assets;
use crate::player::Weapon;
use macroquad::prelude::*;

// Структура телефона
pub struct Phone {
    pub charge: f32,
    pub is_get: bool,
}

pub struct Bullet {
    pub pos: Vec2,
    pub dir: Vec2,
    pub speed: f32,
    pub lifetime: f32,
}

pub struct DroppedWeapon {
    pub pos: Vec2,
    pub weapon: Weapon,
    pub ammo: u32,
    pub rotation: f32,
}

impl DroppedWeapon {
    pub fn new(pos: Vec2, weapon: Weapon, ammo: u32, rotation: f32) -> Self {
        Self {
            pos,
            weapon,
            ammo,
            rotation,
        }
    }

    pub fn collider(&self) -> Rect {
        Rect::new(self.pos.x - 45.0, self.pos.y - 45.0, 90.0, 90.0)
    }

    pub fn draw(&self, assets: &Assets) {
        let sprite_idx = match self.weapon {
            Weapon::Pipe => 0.0,
            Weapon::Knife => 1.0,
            Weapon::Pistol => 2.0,
            Weapon::Rifle => 3.0,
            _ => return,
        };

        let frame_size = 48.0;
        let scale = 2.4;
        let scaled_size = frame_size * scale;

        draw_texture_ex(
            &assets.weapons,
            self.pos.x - scaled_size / 2.0,
            self.pos.y - scaled_size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(scaled_size, scaled_size)),
                source: Some(Rect::new(
                    sprite_idx * frame_size,
                    0.0,
                    frame_size,
                    frame_size,
                )),
                rotation: self.rotation,
                pivot: Some(self.pos),
                ..Default::default()
            },
        );
    }
}

impl Phone {
    // Обновление
    pub fn update(&mut self, delta_time: f32) {
        self.charge = (self.charge - 0.2 * delta_time).max(0.0);

        if is_key_pressed(KeyCode::Q) {
            self.is_get = !self.is_get;
        }
    }

    // Отрисовка
    pub fn draw(&self, assets: &Assets, font_idx: usize) {
        let current_font = assets.get_font(font_idx);
        if self.is_get {
            draw_texture_ex(
                &assets.phone,
                60.0,
                screen_height() - 363.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(128.0, 256.0)),
                    ..Default::default()
                },
            );
            draw_text_ex(
                &format!("{:.0}%", self.charge),
                155.0,
                screen_height() - 345.0,
                TextParams {
                    font: Some(current_font),
                    font_size: 16,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }
    }
}

impl Bullet {
    pub fn new(pos: Vec2, dir: Vec2) -> Self {
        Self {
            pos,
            dir: dir.normalize_or_zero(),
            speed: 3000.0,
            lifetime: 1.5,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.dir * self.speed * dt;
        self.lifetime -= dt;
    }

    pub fn collider(&self) -> Rect {
        Rect::new(self.pos.x - 2.0, self.pos.y - 2.0, 3.0, 3.0)
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2.0, YELLOW);
    }
}
