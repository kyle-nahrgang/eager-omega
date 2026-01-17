use ::rand::Rng;
use macroquad::prelude::*;

use crate::human::{HairStyle, Human};

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: i32 = 32;
const MAP_HEIGHT: i32 = 18;

// --- Player animation constants ---
const FRAME_WIDTH: f32 = 16.0;
const FRAME_HEIGHT: f32 = 16.0;
const FRAME_COUNT: u16 = 9;
const FRAME_TIME: f32 = 0.15;

pub mod human;

#[macroquad::main("Simple Tilemap")]
async fn main() {
    // Load tileset
    let tileset = load_texture("assets/Tileset/spr_tileset_sunnysideworld_16px.png")
        .await
        .unwrap();
    tileset.set_filter(FilterMode::Nearest);

    // Load player idle animation
    let player_texture = load_texture("assets/Characters/Human/IDLE/base_idle_strip9.png")
        .await
        .unwrap();
    player_texture.set_filter(FilterMode::Nearest);

    // Random tilemap
    let mut rng = ::rand::thread_rng();
    let tiles: Vec<u16> = (0..(MAP_WIDTH * MAP_HEIGHT))
        .map(|_| rng.gen_range(1..=5))
        .collect();

    // Player state
    let mut player = Human::new(vec2(100.0, 100.0), 60.0, HairStyle::Bowl).await;

    // Camera
    let mut camera = Camera2D::from_display_rect(Rect::new(
        0.0,
        MAP_HEIGHT as f32 * TILE_SIZE / 4.0,
        MAP_WIDTH as f32 * TILE_SIZE / 4.0,
        -(MAP_HEIGHT as f32 * TILE_SIZE) / 4.0,
    ));

    loop {
        let dt = get_frame_time();

        player.update(dt);

        clear_background(BLACK);

        camera.target = vec2(player.position.x + 24.0, player.position.y + 16.0);

        set_camera(&camera);

        // --- Draw tilemap ---
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let idx = (y * MAP_WIDTH + x) as usize;
                let tile = tiles[idx] + 129;

                draw_texture_ex(
                    &tileset,
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    WHITE,
                    DrawTextureParams {
                        source: Some(tile_uv(tile)),
                        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                );
            }
        }

        player.draw();

        set_default_camera();
        next_frame().await;
    }
}

/* ================= TILE UV HELPER ================= */

fn tile_uv(tile: u16) -> Rect {
    // Tileset has 64 columns
    let tiles_per_row = 64;
    let tile_index = tile - 1; // Tiled counts tiles from 1
    let x = (tile_index % tiles_per_row) as f32 * TILE_SIZE;
    let y = (tile_index / tiles_per_row) as f32 * TILE_SIZE;

    Rect::new(x, y, TILE_SIZE, TILE_SIZE)
}
