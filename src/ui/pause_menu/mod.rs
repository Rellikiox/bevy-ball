mod components;
mod layout;
mod systems;

use crate::game::common::systems::despawn_entities;
use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;
use components::*;
use layout::*;
use systems::*;

const NORMAL_BUTTON_BG_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVER_BUTTON_BG_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON_BG_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SimulationState::Paused),
            spawn_pause_menu.run_if(in_state(AppState::Game)),
        )
        .add_systems(
            OnExit(SimulationState::Paused),
            despawn_entities::<PauseMenu>,
        )
        .add_systems(
            Update,
            (
                interact_with_resume_button,
                interact_with_main_menu_button,
                interact_with_quit_button,
            )
                .run_if(in_state(SimulationState::Paused)),
        );
    }
}
