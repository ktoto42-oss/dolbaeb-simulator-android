use macroquad::prelude::*;

// Структура текстур
pub struct Assets {
    pub player: Texture2D,
    pub phone: Texture2D,
    pub font_pixelify: Font,
    pub font_press_start_2p: Font,
    pub font_times_new_roman: Font,
    pub font_tiny5: Font,
}

impl Assets {
    // Загрузка текстур
    pub async fn load() -> Self {
        // Загрузка текстур
        let player = load_texture("assets/player_tileset.png").await.unwrap();
        let phone = load_texture("assets/phone.png").await.unwrap();

        // Загрузка шрифтов
        let font_pixelify = load_ttf_font("assets/font_pixelify.ttf").await.unwrap();
        let font_press_start_2p = load_ttf_font("assets/font_press_start_2p.ttf").await.unwrap();
        let font_times_new_roman = load_ttf_font("assets/font_times_new_roman.ttf").await.unwrap();
        let font_tiny5 = load_ttf_font("assets/font_tiny5.ttf").await.unwrap();

        // Отключение размытия для пиксель арта
        player.set_filter(FilterMode::Nearest);
        phone.set_filter(FilterMode::Nearest);

        Self {
            player,
            phone,
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
