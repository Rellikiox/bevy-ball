use bevy::prelude::*;

use super::resources::*;
use crate::game::player::resources::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Current score is {}", score.value)
    }
}

pub fn update_highscores(
    mut game_over_er: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_er.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("Current highscores:");
        for (player, score) in high_scores.scores.iter() {
            println!("\t{}: {}", player, score);
        }
    }
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}
