use bevy::app::AppExit;
use bevy::window::PresentMode;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_COUNT: usize = 1;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const STAR_COUNT: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
const STAR_SPAWN_TIME: f32 = 1.0;
const ENEMY_SPAWN_TIME: f32 = 5.0;

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
        .add_systems(
            Startup,
            (
                spawn_player,
                spawn_camera,
                startup_spawn_enemies,
                startup_spawn_stars,
            ),
        )
        .add_systems(
            Update,
            (
                player_movement,
                confine_window_bound,
                enemy_movement,
                update_enemy_direction,
                enemy_bounce,
                enemy_hit_player,
                collect_stars,
                update_score,
                tick_spawn_timers,
                spawn_stars_over_time,
                spawn_enemies_over_time,
                exit_game,
                handle_game_over,
                update_highscores,
                high_scores_updated,
            ),
        )
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Component)]
pub struct WindowBound {
    radius: f32,
}

#[derive(Resource)]
pub struct Score {
    value: u32,
}

impl Default for Score {
    fn default() -> Score {
        return Score { value: 0 };
    }
}

#[derive(Resource)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        return HighScores { scores: Vec::new() };
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        return StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        };
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        return EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        };
    }
}

#[derive(Event)]
pub struct GameOver {
    score: u32,
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
        WindowBound {
            radius: PLAYER_SIZE / 2.0,
        },
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

fn clamp<T: std::cmp::PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        return min;
    }
    if input > max {
        return max;
    }
    return input;
}

pub fn startup_spawn_enemies(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    spawn_enemies(commands, window_query, asset_server, ENEMY_COUNT);
}

pub fn startup_spawn_stars(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    spawn_stars(commands, window_query, asset_server, STAR_COUNT);
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

pub fn confine_window_bound(
    mut query: Query<(&mut Transform, &WindowBound)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, window_bound) in query.iter_mut() {
        let x_min = 0.0 + window_bound.radius;
        let y_min = 0.0 + window_bound.radius;
        let x_max = window.width() - window_bound.radius;
        let y_max = window.height() - window_bound.radius;

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
            let hit_distance = PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0;

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

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Current score is {}", score.value)
    }
}

fn tick_spawn_timers(
    mut star_timer: ResMut<StarSpawnTimer>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    star_timer.timer.tick(time.delta());
    enemy_timer.timer.tick(time.delta());
}

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
) {
    if enemy_timer.timer.finished() {
        spawn_enemies(commands, window_query, asset_server, 1);
    }
}

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn handle_game_over(mut game_over_er: EventReader<GameOver>) {
    for event in game_over_er.read() {
        println!("Your final score is: {}", event.score);
    }
}

fn update_highscores(mut game_over_er: EventReader<GameOver>, mut high_scores: ResMut<HighScores>) {
    for event in game_over_er.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("Current highscores:");
        for (player, score) in high_scores.scores.iter() {
            println!("\t{}: {}", player, score);
        }
    }
}
