use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

mod bevy_helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TiledPlugin::default())
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera (required by Bevy)
    commands.spawn(Camera2d);

    // Load a map then spawn it
    commands.spawn((
        // Only the [`TiledMap`] component is actually required to spawn a map.
        TiledMap(asset_server.load("Maps/sample.tmx")),
        // But you can add extra components to change the defaults settings and how
        // your map is actually displayed
        TilemapAnchor::Center,
    ));
}
