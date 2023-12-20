mod common;
mod enemy;
pub mod player;
mod playing_area;
mod score;
mod stars;

use bevy::prelude::*;

use common::CommonPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use playing_area::PlayingAreaPlugin;
use score::ScorePlugin;
use stars::StarsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CommonPlugin,
            EnemyPlugin,
            PlayerPlugin,
            PlayingAreaPlugin,
            StarsPlugin,
            ScorePlugin,
        ));
    }
}
