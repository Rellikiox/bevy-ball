use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::STAR_COUNT;
use super::STAR_SIZE;
use crate::game::player::components::*;
use crate::game::player::PLAYER_SIZE;
use crate::game::score::resources::*;
use crate::game::stars::components::*;
use crate::game::stars::resources::*;

pub fn spawn_stars_over_time(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_timer: Res<StarSpawnTimer>,
) {
    if star_timer.timer.finished() {
        spawn_stars(commands, window_query, asset_server, 1);
    }
}

fn spawn_stars(
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
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn startup_spawn_stars(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    spawn_stars(commands, window_query, asset_server, STAR_COUNT);
}

pub fn collect_stars(
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut commands: Commands,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let hit_distance = PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0;

            if distance < hit_distance {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
                    ..default()
                });
                commands.entity(star_entity).despawn();
                score.value += 1
            }
        }
    }
}
