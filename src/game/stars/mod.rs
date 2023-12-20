mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub const STAR_COUNT: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (startup_spawn_stars,))
            .add_systems(Update, (collect_stars, spawn_stars_over_time))
            .init_resource::<StarSpawnTimer>();
    }
}
