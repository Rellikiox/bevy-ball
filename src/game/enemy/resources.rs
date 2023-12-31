use bevy::prelude::*;

use super::ENEMY_SPAWN_TIME;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        return EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        };
    }
}

#[derive(Resource)]
pub struct SpawnedEnemies {
    pub value: usize,
}

impl Default for SpawnedEnemies {
    fn default() -> SpawnedEnemies {
        return SpawnedEnemies { value: 0 };
    }
}
