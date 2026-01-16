use ::rand::{Rng, SeedableRng, rngs::StdRng};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: i32 = 256;
const MAP_HEIGHT: i32 = 256;
const SEED: u32 = 1337;

const TILES_PER_ROW: i32 = 64;

#[derive(Clone, Copy)]
enum Tile {
    Grass1 = 65,
    Grass2,
    Stone1,
    Water1,
    Sand1,
    Dirt1,
    Sand2,
    Sand3,
    Sand4,
}

struct Island {
    x: f32,
    y: f32,
    radius: f32,
}

fn generate_islands(seed: u32) -> Vec<Island> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let count = rng.gen_range(6..12);

    (0..count)
        .map(|_| Island {
            x: rng.gen_range(0.0..MAP_WIDTH as f32),
            y: rng.gen_range(0.0..MAP_HEIGHT as f32),
            radius: rng.gen_range(40.0..90.0),
        })
        .collect()
}

fn island_height(x: i32, y: i32, islands: &[Island], noise: &Perlin) -> f32 {
    let mut h: f32 = -1.0;

    for island in islands {
        let dx = x as f32 - island.x;
        let dy = y as f32 - island.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < island.radius {
            let falloff: f32 = 1.0 - dist / island.radius;
            h = h.max(falloff);
        }
    }

    let n = noise.get([x as f64 * 0.05, y as f64 * 0.05]) as f32 * 0.2;
    h + n
}

fn height_to_tile(h: f32) -> Tile {
    if h < 0.0 {
        Tile::Water1
    } else if h < 0.15 {
        Tile::Sand2
    } else if h < 0.4 {
        Tile::Dirt1
    } else {
        Tile::Grass2
    }
}

fn tile_uv(tile: Tile) -> Rect {
    let index = tile as i32;
    let x = index % TILES_PER_ROW;
    let y = index / TILES_PER_ROW;

    Rect::new(
        x as f32 * TILE_SIZE,
        y as f32 * TILE_SIZE,
        TILE_SIZE,
        TILE_SIZE,
    )
}

#[macroquad::main("Islands")]
async fn main() {
    let tileset = load_texture("assets/Tileset/spr_tileset_sunnysideworld_16px.png")
        .await
        .unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let islands = generate_islands(SEED);
    let noise = Perlin::new(SEED);

    let mut camera = Camera2D {
        zoom: vec2(2.0 / screen_width(), -2.0 / screen_height()),
        target: vec2(
            MAP_WIDTH as f32 * TILE_SIZE / 2.0,
            MAP_HEIGHT as f32 * TILE_SIZE / 2.0,
        ),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        /* ---------- CAMERA MOVEMENT ---------- */
        let speed = 500.0 * get_frame_time();

        if is_key_down(KeyCode::W) {
            camera.target.y += speed;
        }
        if is_key_down(KeyCode::S) {
            camera.target.y -= speed;
        }
        if is_key_down(KeyCode::A) {
            camera.target.x -= speed;
        }
        if is_key_down(KeyCode::D) {
            camera.target.x += speed;
        }

        let (_, scroll) = mouse_wheel();
        if scroll != 0.0 {
            let zoom = 1.0 + scroll * 0.1;
            camera.zoom *= zoom;
            camera.zoom.x = camera.zoom.x.clamp(0.0005, 0.02);
            camera.zoom.y = camera.zoom.y.clamp(-0.02, -0.0005);
        }

        set_camera(&camera);

        /* ---------- DRAW MAP ---------- */
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let h = island_height(x, y, &islands, &noise);
                let tile = height_to_tile(h);

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

        draw_text(
            "WASD to move | Mouse wheel to zoom",
            20.0,
            30.0,
            24.0,
            WHITE,
        );

        next_frame().await;
    }
}
