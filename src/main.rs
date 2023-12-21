mod game;
mod systems;
mod ui;

use bevy::prelude::*;
use bevy::window::PresentMode;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;
use systems::*;
use ui::UIPlugin;

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
        // .add_plugins(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        .add_plugins((GamePlugin, UIPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over, app_state_transitions))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
