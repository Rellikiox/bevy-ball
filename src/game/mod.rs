pub mod common;
pub mod enemy;
pub mod player;
mod playing_area;
pub mod score;
mod stars;
mod systems;

use bevy::prelude::*;

use common::CommonPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use playing_area::PlayingAreaPlugin;
use score::ScorePlugin;
use stars::StarsPlugin;
use systems::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugins((
                CommonPlugin,
                EnemyPlugin,
                PlayerPlugin,
                PlayingAreaPlugin,
                StarsPlugin,
                ScorePlugin,
            ))
            .add_systems(
                Update,
                toggle_simulation_state.run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
