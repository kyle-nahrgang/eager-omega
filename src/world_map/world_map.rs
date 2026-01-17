use macroquad::prelude::*;

use crate::world_map::{island::Island, layer::Layer, ocean::Ocean};

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: i32 = 64;
const MAP_HEIGHT: i32 = 64;
const MAX_HEIGHT_TILES: f32 = 8.0;

pub struct WorldMap {
    texture: Texture2D,
    view_width: f32,
    view_height: f32,
    pub layers: Vec<Box<dyn Layer>>,
    pub camera: Camera2D,
    pub start_location: Vec2,
}

impl WorldMap {
    pub async fn new() -> Self {
        let texture = load_texture("assets/Tileset/spr_tileset_sunnysideworld_16px.png")
            .await
            .unwrap();
        texture.set_filter(FilterMode::Nearest);

        let view_height = MAX_HEIGHT_TILES * TILE_SIZE; // max 5 tiles high
        let aspect_ratio = screen_width() / screen_height();
        let view_width = view_height * aspect_ratio;

        let camera =
            Camera2D::from_display_rect(Rect::new(0.0, view_height, view_width, -(view_height)));

        let island = Island::new(MAP_WIDTH as usize, MAP_HEIGHT as usize);

        let start_location = island.center.clone();

        Self {
            texture,
            camera,
            view_height,
            view_width,
            layers: vec![
                Box::new(Ocean::new(MAP_WIDTH as usize, MAP_HEIGHT as usize)),
                Box::new(island),
            ],
            start_location,
        }
    }

    pub fn is_collision(&self, position: Vec2, size: Vec2) -> bool {
        for layer in &self.layers[1..] {
            if layer.is_collision(position, size) {
                return true;
            }
        }

        false
    }

    pub fn draw(&self) {
        // Compute the visible area in world coordinates
        let half_screen = vec2(self.view_width, self.view_height as f32 / 2.0);
        let view_left = self.camera.target.x - half_screen.x / self.camera.zoom.x;
        let view_top = self.camera.target.y - half_screen.y / self.camera.zoom.y;
        let view_right = self.camera.target.x + half_screen.x / self.camera.zoom.x;
        let view_bottom = self.camera.target.y + half_screen.y / self.camera.zoom.y;

        // Convert to tile indices
        let start_x = (view_left / TILE_SIZE).floor().max(0.0) as usize;
        let start_y = (view_top / TILE_SIZE).floor().max(0.0) as usize;
        let end_x = (view_right / TILE_SIZE).ceil().min(MAP_WIDTH as f32) as usize;
        let end_y = (view_bottom / TILE_SIZE).ceil().min(MAP_HEIGHT as f32) as usize;

        for y in start_y..end_y {
            for x in start_x..end_x {
                for layer in &self.layers {
                    if let Some(tile) = layer.get_tile(x, y) {
                        let rect = self.tile_uv(tile as u32);

                        if rect.is_none() {
                            continue; // Empty tile
                        }

                        draw_texture_ex(
                            &self.texture,
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            WHITE,
                            DrawTextureParams {
                                source: rect,
                                dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
    }

    fn tile_uv(&self, tile: u32) -> Option<Rect> {
        // Tileset has 64 columns
        let tiles_per_row = 64;
        let tile_index = tile - 1; // Tiled counts tiles from 1
        let x = (tile_index % tiles_per_row) as f32 * TILE_SIZE;
        let y = (tile_index / tiles_per_row) as f32 * TILE_SIZE;

        Some(Rect::new(x, y, TILE_SIZE, TILE_SIZE))
    }
}
