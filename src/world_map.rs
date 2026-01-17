use ::rand::Rng;
use macroquad::prelude::*;

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: i32 = 32;
const MAP_HEIGHT: i32 = 18;
const VIEW_WIDTH: f32 = MAP_WIDTH as f32 * TILE_SIZE / 4.0;
const VIEW_HEIGHT: f32 = MAP_HEIGHT as f32 * TILE_SIZE / 4.0;

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

    pub fn is_collision(&self, position: Vec2) -> bool {
        let tile_x = (position.x / TILE_SIZE).floor() as i32;
        let tile_y = (position.y / TILE_SIZE).floor() as i32;

        if tile_x < -1 || tile_x >= (MAP_WIDTH - 2) || tile_y < -1 || tile_y >= (MAP_HEIGHT - 2) {
            return true; // Out of bounds is considered a collision
        }

        false
    }

    pub fn draw(&self, camera: &Camera2D) {
        // Compute the visible area in world coordinates
        let half_screen = vec2(VIEW_WIDTH, VIEW_HEIGHT as f32 / 2.0);
        let view_left = camera.target.x - half_screen.x / camera.zoom.x;
        let view_top = camera.target.y - half_screen.y / camera.zoom.y;
        let view_right = camera.target.x + half_screen.x / camera.zoom.x;
        let view_bottom = camera.target.y + half_screen.y / camera.zoom.y;

        // Convert to tile indices
        let start_x = (view_left / TILE_SIZE).floor().max(0.0) as usize;
        let start_y = (view_top / TILE_SIZE).floor().max(0.0) as usize;
        let end_x = (view_right / TILE_SIZE).ceil().min(MAP_WIDTH as f32) as usize;
        let end_y = (view_bottom / TILE_SIZE).ceil().min(MAP_HEIGHT as f32) as usize;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = (y * MAP_WIDTH as usize + x) as usize;
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
