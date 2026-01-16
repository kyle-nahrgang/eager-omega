use std::f32::consts::PI;
use std::ops::Range;

use bevy::image::{ImageArrayLayout, ImageLoaderSettings};
use bevy::input::mouse::{AccumulatedMouseScroll, MouseWheel};
use bevy::prelude::*;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};

const TILE_SIZE: f32 = 16.0;
const MAP_WIDTH: i32 = 512;
const MAP_HEIGHT: i32 = 512;
const SEED: u32 = 1337;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(WorldSeed(SEED))
        .insert_resource(CameraSettings {
            orthographic_viewport_height: 5.,
            // In orthographic projections, we specify camera scale relative to a default value of 1,
            // in which one unit in world space corresponds to one pixel.
            orthographic_zoom_range: 0.1..10.0,
            // This value was hand-tuned to ensure that zooming in and out feels smooth but not slow.
            orthographic_zoom_speed: 0.2,
            // Perspective projections use field of view, expressed in radians. We would
            // normally not set it to more than π, which represents a 180° FOV.
            perspective_zoom_range: (PI / 5.)..(PI - 0.2),
            // Changes in FOV are much more noticeable due to its limited range in radians
            perspective_zoom_speed: 0.05,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_movement, zoom))
        .run();
}

/* ---------------- RESOURCES ---------------- */

#[derive(Resource)]
struct WorldSeed(u32);

#[derive(Debug, Resource)]
struct CameraSettings {
    /// The height of the viewport in world units when the orthographic camera's scale is 1
    pub orthographic_viewport_height: f32,
    /// Clamp the orthographic camera's scale to this range
    pub orthographic_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub orthographic_zoom_speed: f32,
    /// Clamp perspective camera's field of view to this range
    pub perspective_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the perspective camera
    pub perspective_zoom_speed: f32,
}

#[derive(Clone, Copy)]
enum Tile {
    Grass,
    Dirt,
    Water,
    Sand,
}

/* ---------------- SETUP ---------------- */
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, seed: Res<WorldSeed>) {
    // Camera
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            ..OrthographicProjection::default_2d()
        }),
        Transform::default(),
        GlobalTransform::default(),
    ));

    let texture_handle: Handle<Image> = asset_server.load_with_settings(
        "Tileset/array_texture.png",
        |settings: &mut ImageLoaderSettings| {
            settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
        },
    );

    let islands = generate_islands(seed.0);
    let noise = Perlin::new(seed.0);

    const CHUNK_SIZE: i32 = 64;
    let chunks_x = MAP_WIDTH / CHUNK_SIZE;
    let chunks_y = MAP_HEIGHT / CHUNK_SIZE;

    for cx in 0..chunks_x {
        for cy in 0..chunks_y {
            // Build tile data for this chunk
            let mut tile_data: Vec<Option<TileData>> =
                Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let world_x = cx * CHUNK_SIZE + x;
                    let world_y = cy * CHUNK_SIZE + y;

                    let height = island_height(world_x, world_y, &islands, &noise);
                    let tile = height_to_tile(height);
                    tile_data.push(Some(TileData {
                        tileset_index: tile as u16,
                        ..Default::default()
                    }));
                }
            }

            // Spawn the chunk entity
            commands.spawn((
                TilemapChunk {
                    chunk_size: UVec2::splat(CHUNK_SIZE as u32),
                    // size of each tile in world space
                    tile_display_size: UVec2::splat(TILE_SIZE as u32),
                    tileset: texture_handle.clone(),
                    ..Default::default()
                },
                TilemapChunkTileData(tile_data),
                // Place the whole chunk in the world
                Transform::from_translation(Vec3::new(
                    (cx * CHUNK_SIZE) as f32 * TILE_SIZE,
                    (cy * CHUNK_SIZE) as f32 * TILE_SIZE,
                    0.0,
                )),
                GlobalTransform::default(),
            ));
        }
    }
}

/* ---------------- ISLAND GENERATION ---------------- */

struct Island {
    x: f32,
    y: f32,
    radius: f32,
}

fn generate_islands(seed: u32) -> Vec<Island> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let count = rng.gen_range(6..12);

    (0..count)
        .map(|_| Island {
            x: rng.gen_range(0.0..MAP_WIDTH as f32),
            y: rng.gen_range(0.0..MAP_HEIGHT as f32),
            radius: rng.gen_range(40.0..90.0),
        })
        .collect()
}

fn island_height(x: i32, y: i32, islands: &Vec<Island>, noise: &Perlin) -> f32 {
    let mut h: f32 = -1.0;

    for island in islands {
        let dx = x as f32 - island.x;
        let dy = y as f32 - island.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < island.radius {
            let falloff: f32 = 1.0 - (dist / island.radius);
            h = h.max(falloff);
        }
    }

    let n = noise.get([x as f64 * 0.05, y as f64 * 0.05]) as f32 * 0.2;
    h + n
}

fn height_to_tile(h: f32) -> Tile {
    if h < 0.0 {
        Tile::Water
    } else if h < 0.15 {
        Tile::Sand
    } else if h < 0.4 {
        Tile::Dirt
    } else {
        Tile::Grass
    }
}

/* ---------------- CAMERA MOVEMENT ---------------- */

fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let speed = 500.0 * time.delta_secs();

    if keys.pressed(KeyCode::KeyW) {
        transform.translation.y += speed;
    }
    if keys.pressed(KeyCode::KeyS) {
        transform.translation.y -= speed;
    }
    if keys.pressed(KeyCode::KeyA) {
        transform.translation.x -= speed;
    }
    if keys.pressed(KeyCode::KeyD) {
        transform.translation.x += speed;
    }
}

fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // Usually, you won't need to handle both types of projection,
    // but doing so makes for a more complete example.
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.orthographic_zoom_speed;
            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
                camera_settings.orthographic_zoom_range.start,
                camera_settings.orthographic_zoom_range.end,
            );
        }
        Projection::Perspective(ref mut perspective) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.perspective_zoom_speed;

            // Adjust the field of view, but keep it within our stated range.
            perspective.fov = (perspective.fov + delta_zoom).clamp(
                camera_settings.perspective_zoom_range.start,
                camera_settings.perspective_zoom_range.end,
            );
        }
        _ => (),
    }
}
