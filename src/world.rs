use crate::player::Player;
use crate::tilemap::MapId;
use crate::tilemap::WorldManager;
use macroquad::prelude::*;

// Состояние игры (где сейчас находится игрок)
#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    InApartment,
    OnStreet,
    MainMenu,
    Settings,
}

// Переключение локаций (при смене переносит игрока в центр новой локации)
pub fn handle_location_switch(
    state: &mut GameState,
    world_manager: &mut WorldManager,
    player: &mut Player,
) {
    if is_key_pressed(KeyCode::Space) {
        *state = match *state {
            GameState::InApartment => {
                world_manager.switch_to(MapId::Street);
                player.x = 200.0;
                player.y = 200.0;
                GameState::OnStreet
            }
            GameState::OnStreet => {
                world_manager.switch_to(MapId::House);
                player.x = 650.0;
                player.y = 650.0;
                GameState::InApartment
            }
            _ => *state,
        };
    }
}

// Цвет заднего фона
pub fn get_bg_color(state: &GameState) -> Color {
    let bg_color = match state {
        GameState::InApartment => DARKGRAY,
        GameState::OnStreet => Color::new(0.1, 0.12, 0.1, 1.0),
        _ => BLACK,
    };

    bg_color
}
