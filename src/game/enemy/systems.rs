use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};

use crate::game::player::components::*;
use crate::game::player::resources::*;
use crate::game::player::PLAYER_SIZE;
use crate::game::playing_area::components::*;
use crate::game::score::resources::*;

pub fn startup_spawn_enemies(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut spawn_count: ResMut<SpawnedEnemies>,
) {
    spawn_enemies(commands, window_query, asset_server, ENEMY_COUNT);
    spawn_count.value += ENEMY_COUNT;
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
    mut commands: Commands,
    mut game_over_ew: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let hit_distance = PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0;

            if distance < hit_distance {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    ..default()
                });
                commands.entity(player_entity).despawn();
                game_over_ew.send(GameOver { score: score.value });
            }
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    count: usize,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..count {
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
            WindowBound {
                radius: ENEMY_SIZE / 2.0,
            },
        ));
    }
}

pub fn spawn_enemies_over_time(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_timer: Res<EnemySpawnTimer>,
    mut spawn_count: ResMut<SpawnedEnemies>,
) {
    if enemy_timer.timer.finished() {
        spawn_enemies(commands, window_query, asset_server, 1);
        spawn_count.value += 1;
    }
}

pub fn insert_spawned_enemies_resources(mut commands: Commands) {
    commands.insert_resource(SpawnedEnemies::default());
}

pub fn remove_spawned_enemies_resources(mut commands: Commands) {
    commands.insert_resource(SpawnedEnemies::default());
}
