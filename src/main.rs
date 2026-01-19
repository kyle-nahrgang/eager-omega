use macroquad::prelude::*;
use macroquad_tiled::load_map;

use crate::characters::{HairStyle, Human};

pub mod characters;
pub mod world_map;

#[macroquad::main("Simple Tilemap")]
async fn main() {
    let texture = load_texture("assets/Tileset/spr_tileset_sunnysideworld_16px.png")
        .await
        .unwrap();
    texture.set_filter(FilterMode::Nearest);

    let mut player = Human::new(Vec2::new(3.5 * 16.0, 3.5 * 16.0), HairStyle::Spikey).await;

    let tiled_map_json = load_string("assets/Maps/sample.json").await.unwrap();
    let tiled_map = load_map(
        tiled_map_json.as_str(),
        &[(
            "../Tileset/spr_tileset_sunnysideworld_16px.png",
            texture.clone(),
        )],
        &[],
    )
    .unwrap();

    let tile_w = tiled_map.raw_tiled_map.tilewidth as f32;
    let tile_h = tiled_map.raw_tiled_map.tileheight as f32;

    let map_width_px = tiled_map.raw_tiled_map.width as f32 * tile_w;
    let map_height_px = tiled_map.raw_tiled_map.height as f32 * tile_h;

    let view_height = 12.0 * tiled_map.raw_tiled_map.tileheight as f32; // max 5 tiles high
    let aspect_ratio = screen_width() / screen_height();
    let view_width = view_height * aspect_ratio;

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.0, view_height, view_width, -(view_height)));

    // rotated jawns: `0xA000018F` is `18F` rotated 90 degrees clockwise

    loop {
        let dt = get_frame_time();

        player.update(dt, &tiled_map);
        camera.target = player.position;

        clear_background(BLACK);

        set_camera(&camera);
        let view = Rect::new(0.0, 0.0, map_width_px, map_height_px);

        tiled_map.draw_tiles("ocean background", view, None);
        tiled_map.draw_tiles("ground", view, None);
        tiled_map.draw_tiles("solids", view, None);
        tiled_map.draw_tiles("shadows", view, None);
        tiled_map.draw_tiles("edges", view, None);
        tiled_map.draw_tiles("ladders", view, None);
        tiled_map.draw_tiles("doors", view, None);

        player.draw();

        tiled_map.draw_tiles("doortops", view, None);
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

        next_frame().await;
    }
}
