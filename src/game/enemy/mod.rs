mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub const ENEMY_COUNT: usize = 1;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    enemy_bounce,
                    enemy_hit_player,
                    spawn_enemies_over_time,
                ),
            )
            .init_resource::<EnemySpawnTimer>();
    }
}
