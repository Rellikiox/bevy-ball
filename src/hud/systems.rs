use bevy::prelude::*;

use super::components::{EnemyTracker, StarTracker};
use crate::game::enemy::resources::SpawnedEnemies;
use crate::game::score::resources::Score;

pub fn update_star_tracker(mut query: Query<&mut Text, With<StarTracker>>, score: Res<Score>) {
    if score.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            text.sections[0].value = format!("{0:.2}", score.value);
        }
    }
}

pub fn update_enemy_tracker(
    mut query: Query<&mut Text, With<EnemyTracker>>,
    spawned_enemies: Res<SpawnedEnemies>,
) {
    if spawned_enemies.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            text.sections[0].value = format!("{0:.2}", spawned_enemies.value);
        }
    }
}
