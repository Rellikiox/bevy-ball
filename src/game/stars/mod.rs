mod components;
pub mod resources;
mod systems;

use crate::game::common::systems::despawn_entities;
use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;
use components::*;
use resources::*;
use systems::*;

pub const STAR_COUNT: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), (startup_spawn_stars,))
            .add_systems(
                Update,
                (collect_stars, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_entities::<Star>)
            .add_event::<StarSpawn>()
            .add_event::<StarPickup>()
            .init_resource::<StarSpawnTimer>();
    }
}
