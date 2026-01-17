use ::rand::Rng;
use macroquad::prelude::*;

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: i32 = 32;
const MAP_HEIGHT: i32 = 18;

pub struct WorldMap {
    texture: Texture2D,
    tiles: Vec<u16>,
}

impl WorldMap {
    pub async fn new() -> Self {
        let texture = load_texture("assets/Tileset/spr_tileset_sunnysideworld_16px.png")
            .await
            .unwrap();
        texture.set_filter(FilterMode::Nearest);

        let mut rng = ::rand::thread_rng();
        let tiles: Vec<u16> = (0..(MAP_WIDTH * MAP_HEIGHT))
            .map(|_| rng.gen_range(1..=5))
            .collect();

        Self { texture, tiles }
    }

    pub fn draw(&self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let idx = (y * MAP_WIDTH + x) as usize;
                let tile = self.tiles[idx] + 129;

                draw_texture_ex(
                    &self.texture,
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    WHITE,
                    DrawTextureParams {
                        source: Some(self.tile_uv(tile)),
                        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn tile_uv(&self, tile: u16) -> Rect {
        // Tileset has 64 columns
        let tiles_per_row = 64;
        let tile_index = tile - 1; // Tiled counts tiles from 1
        let x = (tile_index % tiles_per_row) as f32 * TILE_SIZE;
        let y = (tile_index / tiles_per_row) as f32 * TILE_SIZE;

        Rect::new(x, y, TILE_SIZE, TILE_SIZE)
    }
}
