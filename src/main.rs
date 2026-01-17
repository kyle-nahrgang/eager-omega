use macroquad::prelude::*;

use crate::human::{HairStyle, Human};

pub mod human;
pub mod world_map;

#[macroquad::main("Simple Tilemap")]
async fn main() {
    let mut world_map = world_map::WorldMap::new().await;

    // Player state
    let mut player = Human::new(vec2(100.0, 100.0), 60.0, HairStyle::Spikey).await;

    loop {
        let dt = get_frame_time();

        player.update(dt, &world_map);

        clear_background(BLACK);

        world_map.camera.target = vec2(player.position.x + 24.0, player.position.y + 16.0);
        set_camera(&world_map.camera);

        world_map.draw();
        player.draw();

        set_default_camera();
        next_frame().await;
    }
}
