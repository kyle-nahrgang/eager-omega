use ::rand::Rng;
use macroquad::prelude::*;

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: i32 = 32;
const MAP_HEIGHT: i32 = 18;

// --- Player animation constants ---
const FRAME_WIDTH: f32 = 16.0;
const FRAME_HEIGHT: f32 = 16.0;
const FRAME_COUNT: u16 = 9;
const FRAME_TIME: f32 = 0.15;

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
    let mut player_pos = vec2(100.0, 100.0);
    let mut current_frame: u16 = 0;
    let mut frame_timer = 0.0;

    // Camera
    let mut camera = Camera2D::from_display_rect(Rect::new(
        0.0,
        MAP_HEIGHT as f32 * TILE_SIZE / 4.0,
        MAP_WIDTH as f32 * TILE_SIZE / 4.0,
        -(MAP_HEIGHT as f32 * TILE_SIZE) / 4.0,
    ));

    loop {
        let dt = get_frame_time();

        // --- Player movement ---
        let mut direction = vec2(0.0, 0.0);
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }

        if direction.length() > 0.0 {
            player_pos += direction.normalize() * 60.0 * dt;
        }

        // --- Animation ---
        frame_timer += dt;
        if frame_timer >= FRAME_TIME {
            frame_timer = 0.0;
            current_frame = (current_frame + 1) % FRAME_COUNT;
        }

        clear_background(BLACK);
        camera.target = player_pos; // center camera on the player
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

        // --- Draw player ---
        draw_texture_ex(
            &player_texture,
            player_pos.x,
            player_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(player_uv(current_frame)),
                dest_size: Some(vec2(FRAME_WIDTH, FRAME_HEIGHT)),
                ..Default::default()
            },
        );

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

fn player_uv(frame: u16) -> Rect {
    let frame_width = 96.0;
    let frame_height = 64.0;

    Rect::new(frame as f32 * frame_width, 0.0, frame_width, frame_height)
}
