use crate::enemy::Enemy;
use crate::objects::DroppedWeapon;
use crate::player::{Player, Weapon};
use crate::world::GameState;
use macroquad::prelude::*;

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
mod enemy;

// Тайл карта
mod tilemap;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dolbaeb Simulator".to_string(),
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Загрузка текстур
    let assets = assets::Assets::load().await;
    let mut world_manager = tilemap::WorldManager::init(2.4);

    // Камера
    let mut camera = Camera2D::default();
    camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());

    // Инициализация игрока
    let mut player = Player::new(650.0, 650.0, 300.0);

    // Спавн телефона
    let mut phone = objects::Phone {
        charge: 100.0,
        is_get: false,
    };

    let mut enemies = vec![
        Enemy::new(vec2(200.0, 100.0), Weapon::Pistol, vec2(1.0, 0.0)),
        Enemy::new(vec2(400.0, 150.0), Weapon::Rifle, vec2(0.0, 1.0)),
        Enemy::new(vec2(500.0, 300.0), Weapon::Fists, vec2(0.0, 0.0)),
        Enemy::new(vec2(300.0, 400.0), Weapon::Knife, vec2(-1.0, 0.0)),
    ];

    let mut dropped_weapons: Vec<DroppedWeapon> = Vec::new();

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
                        0 => {
                            state = GameState::InApartment;
                        } // Старт игры
                        1 => {
                            // В настройки
                            previous_state = GameState::MainMenu;
                            state = GameState::Settings;
                            settings_idx = 0;
                        }
                        2 => {
                            break;
                        } // Выход
                        _ => {}
                    }
                }
            }

            GameState::Settings => {
                // Отрисовка настроек
                ui::draw_settings_menu(&assets, settings_idx, font_idx, fullscreen);

                // Навигация в настройках
                if is_key_pressed(KeyCode::W) {
                    settings_idx = if settings_idx == 0 {
                        3
                    } else {
                        settings_idx - 1
                    };
                }
                if is_key_pressed(KeyCode::S) {
                    settings_idx = if settings_idx == 3 {
                        0
                    } else {
                        settings_idx + 1
                    };
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
                            // Переключение шрифта
                            font_idx = if font_idx == 3 { 0 } else { font_idx + 1 };
                        }
                        2 => {
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
                    // Навигация в паузе
                    if is_key_pressed(KeyCode::W) {
                        pause_idx = if pause_idx == 0 { 3 } else { pause_idx - 1 };
                    }
                    if is_key_pressed(KeyCode::S) {
                        pause_idx = if pause_idx == 3 { 0 } else { pause_idx + 1 };
                    }

                    // Подтверждение
                    if is_key_pressed(KeyCode::Enter) {
                        match pause_idx {
                            0 => {
                                is_paused = false;
                            } // Продолжить
                            1 => {
                                player.restart();
                                is_paused = false;
                            }
                            2 => {
                                previous_state = state; // Текущая локация
                                state = GameState::Settings; // Переключение на настройки
                                settings_idx = 0;
                            }
                            3 => {
                                // Выход в главное меню
                                state = GameState::MainMenu;
                                is_paused = false;
                                menu_idx = 0;
                            }
                            _ => {}
                        }
                    }
                }

                // Обновление игры
                if !is_paused {
                    player.handle_input(
                        delta_time,
                        &world_manager,
                        &camera,
                        &mut dropped_weapons,
                        &assets,
                    );
                    player.update_rotation(&camera);
                    player.location_restriction(world_manager.get_active());
                    world_manager.update_flow_field(vec2(player.x, player.y));
                    player.update_bullets(world_manager.get_active(), delta_time);
                    phone.update(delta_time);
                    for enemy in &mut enemies {
                        enemy.update(
                            &mut player,
                            &world_manager,
                            &mut dropped_weapons,
                            delta_time,
                            &assets,
                        );
                    }
                    world::handle_location_switch(&mut state, &mut world_manager, &mut player);
                    camera.target = vec2(player.x, player.y);
                }

                // Отрисовка мира
                let target_visible_height = 600.0;
                let zoom_y = 2.0 / target_visible_height;
                let zoom_x = zoom_y * (screen_height() / screen_width());
                camera.zoom = vec2(zoom_x, zoom_y);

                clear_background(world::get_bg_color(&state));

                set_camera(&camera);

                world_manager.draw();
                player.draw_bullets();
                for enemy in &enemies {
                    enemy.draw(&assets);
                }
                for item in &dropped_weapons {
                    item.draw(&assets);
                }

                player.draw(&assets);

                // Статичный интерфейс
                ui::draw_ui(&assets, font_idx, &player);
                phone.draw(&assets, font_idx);

                if player.is_dead {
                    ui::draw_dead_menu(&assets, font_idx);
                    if is_key_pressed(KeyCode::R) {
                        player.restart();
                    }
                }

                // Повторыный вызов паузы чтобы она перекрывала интерфейс
                if is_paused {
                    ui::draw_pause_menu(&assets, pause_idx, font_idx);
                }
            }
        }
        next_frame().await
    }
}
