mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_score, update_highscores, high_scores_updated),
        )
        .init_resource::<Score>()
        .init_resource::<HighScores>();
    }
}
