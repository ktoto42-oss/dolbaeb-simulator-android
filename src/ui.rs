use macroquad::prelude::*;
use crate::assets::Assets;

// Отрисовка главного меню
pub fn draw_main_menu(assets: &Assets, selected_idx: usize, font_idx: usize) {
    // Фиксация камеры
    set_default_camera();
    // Отчистка фона
    clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

    // Текущий шрифт
    let current_font = assets.get_font(font_idx);

    // Отрисовка названия
    draw_text_ex(
        "DOLBAEB SIMULATOR",
        screen_width() / 2.0 - 240.0,
        screen_height() / 2.0 - 120.0,
        TextParams { font: Some(current_font), font_size: 50, color: RED, ..Default::default() }
    );

    // Варианты главного меню
    let options = ["PLAY", "SETTINGS", "EXIT"];

    // Отрисовка вариантов
    for (i, option) in options.iter().enumerate() {
        let is_selected = i == selected_idx;
        let color = if is_selected { YELLOW } else { WHITE };
        let text = if is_selected { format!("> {}", option) } else { option.to_string() };

        draw_text_ex(
            &text,
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 + (i as f32 * 45.0),
            TextParams { font: Some(current_font), font_size: 28, color, ..Default::default() }
        );
    }
}

// Отрисовка меню настроек
pub fn draw_settings_menu(assets: &Assets, selected_idx: usize, font_idx: usize, fullscreen: bool, sound_on: bool) {
    // Фиксация камеры
    set_default_camera();
    // Отчистка фона
    clear_background(Color::new(0.03, 0.03, 0.03, 1.0));

    // Текущий шрифт
    let current_font = assets.get_font(font_idx);

    // Отрисовка загаловка
    draw_text_ex(
        "SETTINGS",
        screen_width() / 2.0 - 110.0,
        screen_height() / 2.0 - 120.0,
        TextParams { font: Some(current_font), font_size: 45, color: RED, ..Default::default() }
    );

    let screen_str = if fullscreen { "FULL SCREEN" } else { "WINDOW" };
    let sound_str = if sound_on { "ON" } else { "OFF" };
    let font_name = assets.get_font_name(font_idx);

    // Варианты в настройках
    let options = [
        format!("SCREEN MODE: {}", screen_str),
        format!("VOLUME: {}", sound_str),
        format!("FONT: {}", font_name),
        "BACK".to_string(),
    ];

    // Отрисовка вариантов
    for (i, option) in options.iter().enumerate() {
        let is_selected = i == selected_idx;
        let color = if is_selected { YELLOW } else { WHITE };
        let text = if is_selected { format!("> {}", option) } else { option.to_string() };

        draw_text_ex(
            &text,
            screen_width() / 2.0 - 160.0,
            screen_height() / 2.0 + (i as f32 * 45.0),
            TextParams { font: Some(current_font), font_size: 24, color, ..Default::default() }
        );
    }
}

// Отрисовка меню паузы
pub fn draw_pause_menu(assets: &Assets, selected_idx: usize, font_idx: usize) {
    // Фиксация камеры
    set_default_camera();

    // Теущий шрифт
    let current_font = assets.get_font(font_idx);

    // Полупрозрачный фон поверх замершей игры
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));

    // Отрисовка заголовка
    draw_text_ex(
        "PAUSE",
        screen_width() / 2.0 - 65.0,
        screen_height() / 2.0 - 120.0,
        TextParams { font: Some(current_font), font_size: 45, color: YELLOW, ..Default::default() }
    );

    // Варианты в паузе
    let options = ["RESUME", "SETTINGS", "TO MAIN MENU"];

    // Отрисовка вариантов
    for (i, option) in options.iter().enumerate() {
        let is_selected = i == selected_idx;
        let color = if is_selected { YELLOW } else { WHITE };
        let text = if is_selected { format!("> {}", option) } else { option.to_string() };

        draw_text_ex(
            &text,
            screen_width() / 2.0 - 130.0,
            screen_height() / 2.0 + (i as f32 * 45.0),
            TextParams { font: Some(current_font), font_size: 24, color, ..Default::default() }
        );
    }
}

// Отрисовка интерфейса (пока заглушка)
pub fn draw_ui() {
    set_default_camera();
}
