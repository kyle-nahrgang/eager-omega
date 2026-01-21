//! Spawn the main level.
use std::collections::HashMap;

use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{Player, PlayerAssets, player},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>()
        .register_type::<SpawnTile>()
        .register_type::<TeleportTile>()
        .register_type::<TeleportDestination>()
        .add_systems(Update, (set_player_spawn_from_tile, player_teleport));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
    #[dependency]
    map: Handle<TiledMapAsset>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            map: assets.load("Maps/sample.tmx"),
        }
    }
}

#[derive(Component, Default, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
#[reflect(Component, Default)]
struct SpawnTile;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
struct TeleportTile {
    world_id: String,
    map_id: String,
    tile_id: String,
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
struct TeleportDestination {
    id: String,
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn((
            Name::new("Level"),
            TiledMap(level_assets.map.clone()),
            TilemapAnchor::Center,
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
            children![
                player(100.0, &player_assets, &mut texture_atlas_layouts),
                (
                    Name::new("Gameplay Music"),
                    music(level_assets.music.clone())
                )
            ],
        ))
        .observe(
            |collider_created: On<TiledEvent<ColliderCreated>>, mut commands: Commands| {
                commands
                    .entity(collider_created.event().origin)
                    .insert(RigidBody::Static);
            },
        );
}

fn set_player_spawn_from_tile(
    mut player_query: Query<&mut Transform, With<Player>>,
    spawn_tile: Query<&GlobalTransform, Added<SpawnTile>>,
) {
    for tile_transform in &spawn_tile {
        let mut spawn_pos = tile_transform.translation();

        for mut player_transform in &mut player_query {
            spawn_pos.x += 8.0;
            spawn_pos.y += 8.0;
            player_transform.translation = spawn_pos;
        }
    }
}

fn player_teleport(
    mut player_query: Query<&mut Transform, With<Player>>,
    tiles: Query<(&TeleportTile, &GlobalTransform), Without<Player>>,
    destinations: Query<(&TeleportDestination, &GlobalTransform)>,
) {
    let tile_size = 16.0;

    for mut player in &mut player_query {
        let player_pos = player.translation;

        for (teleport, tile_transform) in &tiles {
            // Get tile center
            let tile_center =
                tile_transform.translation() + Vec3::new(tile_size * 0.5, -tile_size * 0.5, 0.0);

            // Check if player is inside this tile (simple AABB check)
            let half_size = Vec2::new(tile_size * 0.5, tile_size * 0.5);
            let delta = Vec2::new(
                (player_pos.x - tile_center.x).abs(),
                (player_pos.y - tile_center.y).abs(),
            );

            if delta.x <= half_size.x && delta.y <= half_size.y {
                for (dest, dest_transform) in &destinations {
                    if dest.id == teleport.tile_id {
                        // Player entered the teleport tile!
                        player.translation = dest_transform.translation();
                        info!("Teleported player to {:?}", dest.id);
                    }
                }
            }
        }
    }
}
