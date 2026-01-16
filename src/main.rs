use macroquad::prelude::*;
use macroquad_platformer::*;

/* ================= CONFIG ================= */

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: usize = 40;
const MAP_HEIGHT: usize = 20;

/* ================= MAIN ================= */

#[macroquad::main("Simple Tilemap")]
async fn main() {
    // Load the tileset
    let tileset = load_texture("assets/Tileset/spr_tileset_sunnysideworld_16px.png")
        .await
        .unwrap();
    tileset.set_filter(FilterMode::Nearest);

    // Create a map filled with the same tile (index 66)
    let mut tiles = vec![66u16; (MAP_WIDTH * MAP_HEIGHT) as usize];

    // Create the collision layer (all solid for demo)
    let colliders = vec![Tile::Solid; (MAP_WIDTH * MAP_HEIGHT) as usize];

    // Create the physics world
    let mut world = World::new();
    world.add_static_tiled_layer(colliders, TILE_SIZE, TILE_SIZE, MAP_WIDTH, 0);

    // Fixed camera
    let camera = Camera2D::from_display_rect(Rect::new(
        0.0,
        MAP_HEIGHT as f32 * TILE_SIZE,
        MAP_WIDTH as f32 * TILE_SIZE,
        -(MAP_HEIGHT as f32 * TILE_SIZE),
    ));

    loop {
        clear_background(BLACK);

        // Set camera
        set_camera(&camera);

        // Draw the tilemap
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let idx = (y * MAP_WIDTH + x) as usize;
                let tile = tiles[idx];

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
