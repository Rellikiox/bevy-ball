use bevy::prelude::*;

use crate::game::enemy::resources::*;
use crate::game::stars::resources::*;

pub fn tick_spawn_timers(
    mut star_timer: ResMut<StarSpawnTimer>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    star_timer.timer.tick(time.delta());
    enemy_timer.timer.tick(time.delta());
}
