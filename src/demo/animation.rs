//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use rand::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    demo::{movement::MovementController, player::PlayerAssets},
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    player_assets: If<Res<PlayerAssets>>,
    mut player_query: Query<(&MovementController, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.intent.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.intent == Vec2::ZERO {
            PlayerAnimationState::Idling
        } else {
            PlayerAnimationState::Walking
        };

        animation.update_state(
            animation_state,
            player_assets.as_ref().actions.get(&animation_state),
        );

        // change the clip if needed
        sprite.image = animation.clip.base_image.clone();
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.frame;
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the
/// animation.
fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: If<Res<PlayerAssets>>,
    mut step_query: Query<&PlayerAnimation>,
) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::rng();
            let random_step = player_assets.steps.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_step));
        }
    }
}

#[derive(Clone, Reflect, Resource, Debug)]
#[reflect(Resource)]
pub struct PlayerAnimationClip {
    pub base_image: Handle<Image>,
    pub hair_image: Option<Handle<Image>>,
    pub frames: usize,
    pub width: u32,
    pub height: u32,
    pub duration: Duration,
}

impl PlayerAnimationClip {
    pub fn new(
        asset_server: &AssetServer,
        base_path: &'static str,
        hair_path: Option<&'static str>,
        frames: usize,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            base_image: asset_server.load_with_settings(
                base_path,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            hair_image: None,
            frames,
            width,
            height,
            duration: Duration::from_millis(50), // 30 FPS?
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    pub frame: usize,
    state: PlayerAnimationState,
    clip: PlayerAnimationClip,
}

#[derive(Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum PlayerAnimationState {
    Idling,
    Walking,
}

impl PlayerAnimation {
    pub fn new(start_clip: &PlayerAnimationClip) -> Self {
        Self {
            timer: Timer::new(start_clip.duration, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
            clip: start_clip.clone(),
        }
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1) % self.clip.frames;
    }

    /// Update animation state if it changes.
    pub fn update_state(
        &mut self,
        state: PlayerAnimationState,
        clip: Option<&PlayerAnimationClip>,
    ) {
        if self.state != state
            && let Some(clip) = clip
        {
            *self = Self {
                timer: Timer::new(clip.duration, TimerMode::Repeating),
                frame: 0,
                state,
                clip: clip.clone(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }
}
