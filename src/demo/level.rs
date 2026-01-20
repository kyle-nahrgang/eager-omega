//! Spawn the main level.
use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{PlayerAssets, player},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
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
