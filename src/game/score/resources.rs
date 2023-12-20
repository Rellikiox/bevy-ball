use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        return Score { value: 0 };
    }
}

#[derive(Resource)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        return HighScores { scores: Vec::new() };
    }
}
