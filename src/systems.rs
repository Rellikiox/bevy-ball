use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::player::resources::GameOver;
use crate::game::SimulationState;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_er: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_er.read() {
        println!("Your final score is: {}", event.score);
        next_app_state.set(AppState::GameOver);
    }
}

pub fn app_state_transitions(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        match app_state.get() {
            AppState::MainMenu => {
                next_app_state.set(AppState::Game);
                println!("Enter Game");
            }
            AppState::Game => {
                next_app_state.set(AppState::MainMenu);
                next_simulation_state.set(SimulationState::Paused);
                println!("Enter MainMenu");
            }
            AppState::GameOver => {
                next_app_state.set(AppState::Game);
                next_simulation_state.set(SimulationState::Paused);
                println!("Enter MainMenu");
            }
        }
    }
}
