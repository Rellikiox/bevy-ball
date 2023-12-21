use bevy::prelude::*;

use super::STAR_SPAWN_TIME;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        return StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        };
    }
}

#[derive(Event)]
pub struct StarSpawn {}

#[derive(Event)]
pub struct StarPickup {}
