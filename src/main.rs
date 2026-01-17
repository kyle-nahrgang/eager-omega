use macroquad::prelude::*;

use crate::characters::{HairStyle, Human};

pub mod characters;
pub mod world_map;

#[macroquad::main("Simple Tilemap")]
async fn main() {
    let mut world_map = world_map::WorldMap::new().await;

    // Player state
    let mut player = Human::new(world_map.start_location, HairStyle::Spikey).await;

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
