mod systems;
pub mod utils;

use bevy::prelude::*;
use systems::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_spawn_timers);
    }
}
