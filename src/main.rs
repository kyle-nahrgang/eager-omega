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

        world_map.camera.target = player.position;
        set_camera(&world_map.camera);

        world_map.draw();
        player.draw();

        let tile_x = (player.position.x / 16.0).floor() as i32;
        let tile_y = (player.position.y / 16.0).floor() as i32;

        draw_rectangle_lines(
            tile_x as f32 * 16.0,
            tile_y as f32 * 16.0,
            16.0,
            16.0,
            2.0,
            RED,
        );

        set_default_camera();

        // Highlight the tile the player is on
        draw_text(
            &format!("{:.2}, {:.2})", player.position.x, player.position.y),
            50.0,
            70.0,
            20.0,
            WHITE,
        );

        draw_text(
            &format!("Tile ({}, {})", tile_x, tile_y),
            50.0,
            90.0,
            20.0,
            WHITE,
        );

        draw_text(
            &format!(
                "{:?}",
                world_map.layers[1].get_tile(tile_x as usize, tile_y as usize)
            ),
            50.0,
            110.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
