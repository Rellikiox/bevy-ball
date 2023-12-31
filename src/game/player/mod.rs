pub mod components;
pub mod resources;
pub mod systems;

use super::SimulationState;
use crate::game::common::systems::*;
use crate::AppState;
use bevy::prelude::*;
use components::*;
use resources::*;
use systems::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), (spawn_player,))
            .add_systems(
                Update,
                (player_movement,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_entities::<Player>)
            .add_event::<GameOver>();
    }
}
