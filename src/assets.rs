use macroquad::prelude::*;

// Структура Ассетов
pub struct Assets {
    pub player: Texture2D,
    pub enemy: Texture2D,
    pub phone: Texture2D,
    pub weapons: Texture2D,
    pub font_pixelify: Font,
    pub font_press_start_2p: Font,
    pub font_times_new_roman: Font,
    pub font_tiny5: Font,
}

impl Assets {
    pub fn load() -> Self {
        // Загрузка текстур
        let player_bytes = include_bytes!("../assets/player_tileset.png");
        let phone_bytes = include_bytes!("../assets/phone.png");
        let enemy_bytes = include_bytes!("../assets/enemy_tileset.png");
        let weapons_bytes = include_bytes!("../assets/weapons.png");

        let player = Texture2D::from_file_with_format(player_bytes, None);
        let phone = Texture2D::from_file_with_format(phone_bytes, None);
        let enemy = Texture2D::from_file_with_format(enemy_bytes, None);
        let weapons = Texture2D::from_file_with_format(weapons_bytes, None);

        // Загрузка шрифтов
        let font_pixelify_bytes = include_bytes!("../assets/font_pixelify.ttf");
        let font_pixelify = load_ttf_font_from_bytes(font_pixelify_bytes).unwrap();

        let font_press_start_2p_bytes = include_bytes!("../assets/font_press_start_2p.ttf");
        let font_press_start_2p = load_ttf_font_from_bytes(font_press_start_2p_bytes).unwrap();

        let font_times_new_roman_bytes = include_bytes!("../assets/font_times_new_roman.ttf");
        let font_times_new_roman = load_ttf_font_from_bytes(font_times_new_roman_bytes).unwrap();

        let font_tiny5_bytes = include_bytes!("../assets/font_tiny5.ttf");
        let font_tiny5 = load_ttf_font_from_bytes(font_tiny5_bytes).unwrap();

        // Отключение размытия для пиксель арта
        player.set_filter(FilterMode::Nearest);
        phone.set_filter(FilterMode::Nearest);
        enemy.set_filter(FilterMode::Nearest);
        weapons.set_filter(FilterMode::Nearest);

        Self {
            player,
            enemy,
            phone,
            weapons,
            font_pixelify,
            font_press_start_2p,
            font_times_new_roman,
            font_tiny5,
        }
    }

    // Возвращает ссылку на шрифт по его индексу
    pub fn get_font(&self, idx: usize) -> &Font {
        match idx {
            1 => &self.font_pixelify,
            2 => &self.font_press_start_2p,
            3 => &self.font_times_new_roman,
            _ => &self.font_tiny5,
        }
    }

    // Возвращает название шрифта для отображения в меню
    pub fn get_font_name(&self, idx: usize) -> &str {
        match idx {
            1 => "PIXELIFY",
            2 => "PRESS START 2P",
            3 => "TIMES NEW ROMAN",
            _ => "TINY 5",
        }
    }
}
