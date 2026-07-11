use macroquad::prelude::*;
use crate::player::Player;
use crate::world::GameState;
use crate::{STREET_WIDTH, STREET_HEIGHT, APT_HEIGHT, APT_WIDTH};

#[derive(PartialEq)]
pub enum EnemyState {
    Patrol,
    Chase,
}

// Структура врагов
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed_patrol: f32,
    pub speed_chase: f32,
    pub rotation: f32,
    pub state: EnemyState,
    pub direction: f32,
    pub vision_radius: f32,
    pub width: f32,
    pub height: f32,
}

impl Enemy {
    pub fn new(pos: [f32; 2]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
            speed_patrol: 80.0,
            speed_chase: 160.0,
            rotation: 0.0,
            state: EnemyState::Patrol,
            direction: 1.0,
            vision_radius: 250.0,
            width: 40.0,
            height: 40.0,
        }
    }

    pub fn update(&mut self, player: &Player, delta_time: f32) {
        let enemy_pos = vec2(self.x, self.y);
        let player_pos = vec2(player.x, player.y);

        // Расстояние до игрока
        let distance_to_player = enemy_pos.distance(player_pos);

        // Условие перехода в режим погони
        if distance_to_player < self.vision_radius {
            self.state = EnemyState::Chase;
        } else {
            // Если игрок убежал далеко возвращается к патрулированию
            if self.state == EnemyState::Chase {
                self.state = EnemyState::Patrol;
            }
        }

        match self.state {
            EnemyState::Patrol => {
                /*
                // Движение между стенами
                self.x += self.direction * self.speed_patrol * delta_time;

                if self.x >= self.patrol_max_x {
                    self.x = self.patrol_max_x;
                    self.direction = -1.0; // Разворот влево
                } else if self.x <= self.patrol_min_x {
                    self.x = self.patrol_min_x;
                    self.direction = 1.0; // Разворот вправо
                }
                */

                // Поворот угла спрайта в сторону движения
                self.rotation = if self.direction > 0.0 { 0.0 } else { std::f32::consts::PI };
            }

            EnemyState::Chase => {
                // Вектор направления к игроку
                let to_player = player_pos - enemy_pos;

                if to_player.length() > 0.0 {
                    let dir = to_player.normalize();

                    // Движение к игроку по обеим осям
                    self.x += dir.x * self.speed_chase * delta_time;
                    self.y += dir.y * self.speed_chase * delta_time;

                    // Вращение врага лицом к игроку
                    self.rotation = dir.y.atan2(dir.x);
                }
            }
        }
    }

    pub fn draw(&self, state: &GameState) {

        // Отрисовка зоны видимости
        if *state != GameState::InApartment {
            draw_circle_lines(self.x, self.y, self.vision_radius, 1.0, Color::new(1.0, 0.0, 0.0, 0.2));

            let color = match self.state {
                EnemyState::Patrol => BLUE,
                EnemyState::Chase => RED,
            };

            // Отрисовка врага
            draw_rectangle_ex(
                self.x - 12.0,
                self.y - 12.0,
                24.0,
                24.0,
                DrawRectangleParams {
                    rotation: self.rotation,
                    color: color,
                    //pivot: Some(vec2(self.x, self.y)),
                    ..Default::default()
                },
            );
        }
    }
}
