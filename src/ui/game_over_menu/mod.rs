mod components;
mod layout;
mod systems;

use crate::game::common::systems::despawn_entities;
use crate::AppState;
use bevy::prelude::*;
use components::*;
use layout::*;
use systems::*;

const NORMAL_BUTTON_BG_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVER_BUTTON_BG_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON_BG_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(AppState::GameOver), despawn_entities::<GameOverMenu>)
            .add_systems(
                Update,
                (
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::GameOver)),
            );
    }
}
