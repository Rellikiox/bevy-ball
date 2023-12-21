mod components;
mod layout;
mod systems;

use crate::game::common::systems::despawn_entities;
use crate::AppState;
use bevy::prelude::*;
use components::*;
use layout::*;
use systems::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(
                Update,
                (update_star_tracker, update_enemy_tracker).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_entities::<HUD>);
        // .add_systems(
        //     Update,
        //     (interact_with_play_button, interact_with_quit_button)
        //         .run_if(in_state(AppState::MainMenu)),
        // );
    }
}
