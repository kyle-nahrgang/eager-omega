//! Player-specific behavior.

use std::collections::HashMap;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    demo::{
        animation::{PlayerAnimation, PlayerAnimationClip, PlayerAnimationState},
        movement::MovementController,
    },
};
use avian2d::prelude::{Collider, LinearVelocity, LockedAxes, RigidBody};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (
            camera_follow,
            record_player_directional_input
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems),
        ),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    pub actions: HashMap<PlayerAnimationState, PlayerAnimationClip>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            actions: HashMap::from_iter([
                (
                    PlayerAnimationState::Idling,
                    PlayerAnimationClip::new(
                        &assets,
                        "Characters/Human/IDLE/base_idle_strip9.png",
                        "Characters/Human/IDLE/spikeyhair_idle_strip9.png",
                        9,
                        96,
                        64,
                    ),
                ),
                (
                    PlayerAnimationState::Walking,
                    PlayerAnimationClip::new(
                        &assets,
                        "Characters/Human/RUN/base_run_strip8.png",
                        "Characters/Human/RUN/spikeyhair_run_strip8.png",
                        8,
                        96,
                        64,
                    ),
                ),
            ]),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}

/// The player character.
pub fn player(
    max_speed: f32,
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::new(96, 64), 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let idle_animation = player_assets
        .actions
        .get(&PlayerAnimationState::Idling)
        .unwrap();
    let player_animation = PlayerAnimation::new(idle_animation);
    (
        Name::new("Player"),
        Player,
        children![
            (
                Name::new("Player Body"),
                PlayerBody,
                Sprite::from_atlas_image(
                    idle_animation.base_image.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: player_animation.frame,
                    },
                ),
                Transform {
                    translation: Vec3::new(0.0, 0.0, -1.0),
                    ..default()
                },
            ),
            (
                Name::new("Player Hair"),
                PlayerHair,
                Sprite::from_atlas_image(
                    idle_animation.hair_image.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: player_animation.frame,
                    },
                )
            ),
        ],
        RigidBody::Dynamic,
        LinearVelocity::default(),
        Collider::circle(1.0),
        LockedAxes::ROTATION_LOCKED,
        Transform {
            translation: Vec3::new(-80.0, -80.0, -150.0),
            ..default()
        },
        MovementController {
            max_speed,
            ..default()
        },
        player_animation,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerBody;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerHair;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
