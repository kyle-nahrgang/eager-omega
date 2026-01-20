//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::prelude::Collider;
use bevy::{
    dev_tools::states::log_transitions,
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    app.add_systems(Update, draw_colliders.run_if(input_pressed(TOGGLE_KEY)));
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn draw_colliders(
    mut commands: Commands,
    colliders: Query<(&Transform, &Collider)>,
    ui_debug_options: Res<UiDebugOptions>,
) {
    if !ui_debug_options.enabled {
        return;
    }

    for (transform, collider) in &colliders {
        if let Some(poly) = collider.shape().as_polyline() {
            for pair in poly.indices() {
                let start = transform.translation
                    + Vec3::new(
                        poly.vertices()[pair[0] as usize][0],
                        poly.vertices()[pair[0] as usize][1],
                        0.0,
                    );
                let end = transform.translation
                    + Vec3::new(
                        poly.vertices()[pair[1] as usize][0],
                        poly.vertices()[pair[1] as usize][1],
                        0.0,
                    );
                let line_length = (end - start).length();
                let line_angle = (end - start).xy().angle_to(Vec2::X);
                commands.spawn((
                    Sprite {
                        color: Color::Srgba(Srgba::rgb(1.0, 0.0, 0.0)),
                        custom_size: Some(Vec2::new(line_length, 2.0)), // 2 pixels thick line
                        ..Default::default()
                    },
                    Transform {
                        translation: (start + end) / 2.0,
                        rotation: Quat::from_rotation_z(line_angle),
                        ..Default::default()
                    },
                ));
            }
        }
    }
}
