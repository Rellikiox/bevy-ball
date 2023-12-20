pub mod components;
mod systems;

use crate::game::enemy::systems::enemy_movement;
use crate::game::player::systems::player_movement;
use bevy::prelude::*;
use systems::*;
pub struct PlayingAreaPlugin;

impl Plugin for PlayingAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            confine_window_bound
                .after(enemy_movement)
                .after(player_movement),
        );
    }
}
