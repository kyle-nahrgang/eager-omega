use ::rand::Rng;
use macroquad::prelude::*;

use crate::human::{HairStyle, Human};

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: i32 = 32;
const MAP_HEIGHT: i32 = 18;

pub mod human;
pub mod world_map;

#[macroquad::main("Simple Tilemap")]
async fn main() {
    let world_map = world_map::WorldMap::new().await;

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

        world_map.draw();
        player.draw();

        set_default_camera();
        next_frame().await;
    }
}
