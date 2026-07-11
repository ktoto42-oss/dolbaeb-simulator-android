use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 32.0;
// ТУТ НИХУЯ НЕ ГОТОВО И ВООБЩЕ ВАЙБКОД, ИДИ НАХУЙ ОТ СЮДА
pub struct TileMap {
    pub tiles: Vec<Vec<u32>>,
}

impl TileMap {
    pub fn new(tiles: Vec<Vec<u32>>) -> Self {
        Self { tiles }
    }

    pub fn draw(&self, texture: &Texture2D) {
        if self.tiles.is_empty() || self.tiles[0].is_empty() {
            return;
        }

        let tiles_per_row = (texture.width() / TILE_SIZE) as u32;
        if tiles_per_row == 0 {
            return;
        }

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile_id) in row.iter().enumerate() {
                let src_x = (tile_id % tiles_per_row) as f32 * TILE_SIZE;
                let src_y = (tile_id / tiles_per_row) as f32 * TILE_SIZE;

                let world_x = x as f32 * TILE_SIZE;
                let world_y = y as f32 * TILE_SIZE;

                draw_texture_ex(
                    texture,
                    world_x,
                    world_y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(src_x, src_y, TILE_SIZE, TILE_SIZE)),
                        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

pub fn create_apartment_map() -> TileMap {
    TileMap::new(vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 2, 2, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 2, 2, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ])
}

pub fn create_street_map() -> TileMap {
    TileMap::new(vec![
        vec![4, 4, 4, 4, 4, 4, 4, 4],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![3, 3, 3, 3, 3, 3, 3, 3],
        vec![3, 3, 3, 3, 3, 3, 3, 3],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![4, 4, 4, 4, 4, 4, 4, 4],
    ])
}
