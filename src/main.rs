use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::window::PresentMode;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_COUNT: usize = 1;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: (1280.0, 1024.0).into(),
            title: "Ballz".to_string(),
            present_mode: PresentMode::Immediate,
            ..default()
        }),
        ..default()
    };
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemies))
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                enemy_bounce,
                enemy_hit_player,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
        ..default()
    });
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let y_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_max = window.height() - half_player_size;

        transform.translation.x = clamp(transform.translation.x, x_min, x_max);
        transform.translation.y = clamp(transform.translation.y, y_min, y_max);
    }
}

fn clamp<T: std::cmp::PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        return min;
    }
    if input > max {
        return max;
    }
    return input;
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_COUNT {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        if transform.translation.x <= x_min || transform.translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if transform.translation.y <= y_min || transform.translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sfx = if random::<f32>() > 0.5 {
                "audio/pluck_001.ogg"
            } else {
                "audio/pluck_002.ogg"
            };
            commands.spawn(AudioBundle {
                source: asset_server.load(sfx),
                ..default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        transform.translation.x = clamp(transform.translation.x, x_min, x_max);
        transform.translation.y = clamp(transform.translation.y, y_min, y_max);
    }
}

pub fn enemy_bounce(mut enemy_query: Query<(&Transform, &mut Enemy)>) {
    let mut iter = enemy_query.iter_combinations_mut();
    while let Some([(transform_1, mut enemy_1), (transform_2, mut enemy_2)]) = iter.fetch_next() {
        let distance = Vec3::abs(transform_1.translation - transform_2.translation);
        if distance.length() <= ENEMY_SIZE {
            let direction_1 = (transform_1.translation - transform_2.translation).normalize();
            enemy_1.direction.x = direction_1.x;
            enemy_1.direction.y = direction_1.y;
            let direction_2 = (transform_2.translation - transform_1.translation).normalize();
            enemy_2.direction.x = direction_2.x;
            enemy_2.direction.y = direction_2.y;
        }
    }
}

pub fn enemy_hit_player(
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let hit_distance = PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0;

            if distance < hit_distance {
                println!("Enemy hit player! Game over!");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    ..default()
                });
                commands.entity(player_entity).despawn();
            }
        }
    }
}
