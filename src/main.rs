use macroquad::prelude::*;
use crate::world::{APT_HEIGHT, APT_WIDTH, STREET_HEIGHT, STREET_WIDTH, GameState};
use crate::player::Player;
use crate::npc::Enemy;

// Игрок
mod player;

// Мир
mod world;

// Интерфейс
mod ui;

// Текстуры
mod assets;

// Объекты
mod objects;

// NPC
mod npc;

#[macroquad::main("Dolbaeb simulator")]
async fn main() {
    // Загрузка текстур
    let assets = assets::Assets::load().await;

    // Камера
    let mut camera = Camera2D::default();
    camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());

    // Инициализация игрока
    let mut player = Player::new(0.0, 0.0, 300.0);

    // Спавн телефона
    let mut phone = objects::Phone {
        charge: 100.0,
        is_get: false,
    };

    let enemyes = [
        [100.0, 100.0],
        [300.0, 300.0],
        [500.0, 500.0],
    ];
    let mut enemy = Enemy::new(enemyes[0]);

    // Состояние
    let mut state = GameState::MainMenu;

    // Флаг паузы
    let mut is_paused = false;

    // Состояния меню и настроек
    let mut state = GameState::MainMenu;
    let mut previous_state = GameState::MainMenu;
    let mut is_paused = false;

    // Индексы выбранных пунктов для каждого меню
    let mut menu_idx = 0;
    let mut pause_idx = 0;
    let mut settings_idx = 0;

    // Текущие настройки (флаги)
    let mut fullscreen = false;
    let mut sound_on = true;
    let mut font_idx = 0;

    // Главный игровой цикл
    loop {
        // Дельта времени (чтобы игра работала одинаково при разном фпс)
        let delta_time = get_frame_time();

        match state {
            GameState::MainMenu => {
                // Отрисовка меню
                ui::draw_main_menu(&assets, menu_idx, font_idx);

                // Навигация (W - вверх S - вниз)
                if is_key_pressed(KeyCode::W) {
                    menu_idx = if menu_idx == 0 { 2 } else { menu_idx - 1 };
                }
                if is_key_pressed(KeyCode::S) {
                    menu_idx = if menu_idx == 2 { 0 } else { menu_idx + 1 };
                }

                // Подтверждение
                if is_key_pressed(KeyCode::Enter) {
                    match menu_idx {
                        0 => { state = GameState::InApartment; } // Старт игры
                        1 => { // В настройки
                            previous_state = GameState::MainMenu;
                            state = GameState::Settings;
                            settings_idx = 0;
                        }
                        2 => { break; } // Выход
                        _ => {}
                    }
                }
            }

            GameState::Settings => {
                // Отрисовка настроек
                ui::draw_settings_menu(&assets, settings_idx, font_idx, fullscreen, sound_on);

                // Навигация в настройках
                if is_key_pressed(KeyCode::W) {
                    settings_idx = if settings_idx == 0 { 3 } else { settings_idx - 1 };
                }
                if is_key_pressed(KeyCode::S) {
                    settings_idx = if settings_idx == 3 { 0 } else { settings_idx + 1 };
                }

                // Подтверждение
                if is_key_pressed(KeyCode::Enter) {
                    match settings_idx {
                        0 => {
                            // Переключение на фулл скрин
                            fullscreen = !fullscreen;
                            set_fullscreen(fullscreen);
                        }
                        1 => {
                            // Выключение звука
                            sound_on = !sound_on;
                        }
                        2 => {
                            // Переключение шрифта
                            font_idx = if font_idx == 3 { 0 } else { font_idx + 1 };
                        }
                        3 => {
                            // Возвращение туда откуда вызванно
                            state = previous_state;
                        }
                        _ => {}
                    }
                }

                // Возвращение туда откуда вызванно (на esc)
                if is_key_pressed(KeyCode::Escape) {
                    state = previous_state;
                }
            }

            _ => {
                // Нажатие ESC вызывает или закрывает паузу
                if is_key_pressed(KeyCode::Escape) {
                    is_paused = !is_paused;
                    pause_idx = 0; // Сброс стрелки паузы на первый пункт
                }

                // Отработка паузы
                if is_paused {
                    ui::draw_pause_menu(&assets, pause_idx, font_idx);

                    // Навигация в паузе
                    if is_key_pressed(KeyCode::W) {
                        pause_idx = if pause_idx == 0 { 2 } else { pause_idx - 1 };
                    }
                    if is_key_pressed(KeyCode::S) {
                        pause_idx = if pause_idx == 2 { 0 } else { pause_idx + 1 };
                    }

                    // Подтверждение
                    if is_key_pressed(KeyCode::Enter) {
                        match pause_idx {
                            0 => { is_paused = false; } // Продолжить
                            1 => {
                                previous_state = state; // Текущая локация
                                state = GameState::Settings; // Переключение на настройки
                                settings_idx = 0;
                            }
                            2 => {
                                // Выход в главное меню
                                state = GameState::MainMenu;
                                is_paused = false;
                                menu_idx = 0;

                                player.x = APT_WIDTH / 2.0;
                                player.y = APT_HEIGHT / 2.0;
                                enemy.x = 200.0;
                                enemy.y = 400.0;
                                enemy.state = npc::EnemyState::Patrol;
                            }
                            _ => {}
                        }
                    }
                }

                // Обновление игры
                if !is_paused {
                    player.handle_input(delta_time);
                    player.update_rotation(&camera);
                    player.location_restriction(&state);
                    phone.update(delta_time);
                    enemy.update(&player, delta_time);

                    world::handle_location_switch(&mut state, &mut player);
                    camera.target = vec2(player.x, player.y);
                }

                // Отрисовка мира
                let target_visible_height = 600.0;
                let zoom_y = 2.0 / target_visible_height;
                let zoom_x = zoom_y * (screen_height() / screen_width());
                camera.zoom = vec2(zoom_x, zoom_y);

                clear_background(world::get_bg_color(&state));

                set_camera(&camera);
                world::draw_world(&state);
                enemy.draw(&state);
                player.draw(&assets);

                // Статичный интерфейс
                ui::draw_ui();
                phone.draw(&assets, font_idx);

                // Повторыный вызов паузы чтобы она перекрывала интерфейс
                if is_paused {
                    ui::draw_pause_menu(&assets, pause_idx, font_idx);
                }
            }
        }
        next_frame().await
    }
}
