use bevy::prelude::*;
use rand::Rng;

const GRAVITY: f32 = -1200.0;
const PLAYER_SPEED: f32 = 300.0;
const JUMP_FORCE: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Procedural Platformer".into(),
                resolution: (960, 540).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_input,
                apply_gravity,
                movement,
                collisions,
                camera_follow,
            ),
        )
        .run();
}

/* ---------------- Components ---------------- */

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Size(Vec2);

/* ---------------- Setup ---------------- */

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
    ));

    // Player
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 1.0),
            custom_size: Some(Vec2::new(30.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 150.0, 1.0),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
        Player,
        Velocity(Vec2::ZERO),
        Size(Vec2::new(30.0, 40.0)),
    ));

    generate_platforms(&mut commands);
}

/* ---------------- Procedural Generation ---------------- */

fn generate_platforms(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let mut x = -400.0;
    let mut y = -100.0;

    for _ in 0..30 {
        let width = rng.gen_range(80.0..160.0);

        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.8, 0.3),
                custom_size: Some(Vec2::new(width, 20.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            Platform,
            Size(Vec2::new(width, 20.0)),
        ));

        x += rng.gen_range(120.0..200.0);
        y += rng.gen_range(-40.0..40.0);
    }
}

/* ---------------- Systems ---------------- */

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let Ok(mut velocity) = query.single_mut() else {
        return;
    };

    velocity.0.x = 0.0;

    if keyboard.pressed(KeyCode::KeyA) {
        velocity.0.x -= PLAYER_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        velocity.0.x += PLAYER_SPEED;
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    let Ok(mut velocity) = query.single_mut() else {
        return;
    };

    velocity.0.y += GRAVITY * time.delta_secs();
}

fn movement(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn collisions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &Size), With<Player>>,
    platform_query: Query<(&Transform, &Size), (With<Platform>, Without<Player>)>,
) {
    let Ok((mut player_tf, mut velocity, player_size)) = player_query.single_mut() else {
        return;
    };

    let player_pos = player_tf.translation.truncate();
    let mut on_ground = false;

    for (platform_tf, platform_size) in &platform_query {
        let platform_pos = platform_tf.translation.truncate();

        let overlap_x =
            (player_pos.x - platform_pos.x).abs() < (player_size.0.x + platform_size.0.x) * 0.5;

        let overlap_y =
            (player_pos.y - platform_pos.y).abs() < (player_size.0.y + platform_size.0.y) * 0.5;

        if overlap_x && overlap_y && velocity.0.y <= 0.0 {
            player_tf.translation.y = platform_pos.y + (player_size.0.y + platform_size.0.y) * 0.5;
            velocity.0.y = 0.0;
            on_ground = true;
        }
    }

    if on_ground && keyboard.just_pressed(KeyCode::Space) {
        velocity.0.y = JUMP_FORCE;
    }
}

fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(player_tf) = player_query.single() else {
        return;
    };

    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };

    camera_tf.translation.x = player_tf.translation.x;
    camera_tf.translation.y = player_tf.translation.y + 50.0;
}
