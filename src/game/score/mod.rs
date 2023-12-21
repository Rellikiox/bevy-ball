mod components;
pub mod resources;
mod systems;

use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::GameOver), remove_score)
            .add_systems(
                Update,
                (update_highscores)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .init_resource::<HighScores>();
    }
}
