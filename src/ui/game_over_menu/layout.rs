use bevy::prelude::*;

use super::components::*;
use super::NORMAL_BUTTON_BG_COLOR;
use crate::game::score::resources::HighScores;
use crate::game::score::resources::Score;

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    high_score: Res<HighScores>,
) {
    let final_score_text = format!("Your final score was {}!", score.value);
    let best_score = match high_score.scores.iter().max_by_key(|s| s.1) {
        Some(score) => score.1,
        None => 0,
    };
    let best_score_text = format!("Your best score so far was {}!", best_score);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.1).into(),
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(40.0),
                        width: Val::Px(400.0),
                        height: Val::Percent(50.0),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_BG_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    build_text(parent, "Game over! :(", &asset_server);
                    build_text(parent, &final_score_text, &asset_server);
                    build_text(parent, &best_score_text, &asset_server);
                    build_button(parent, RestartButton {}, "Restart", &asset_server);
                    build_button(parent, MainMenuButton {}, "Main Menu", &asset_server);
                    build_button(parent, QuitButton {}, "Quit", &asset_server);
                });
        });
}

pub fn build_button<T: Component>(
    parent: &mut ChildBuilder,
    marker: T,
    text: &str,
    asset_server: &Res<AssetServer>,
) -> Entity {
    return parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                ..default()
            },
            marker,
        ))
        .with_children(|parent| {
            build_text(parent, text, asset_server);
        })
        .id();
}

pub fn build_text(
    parent: &mut ChildBuilder,
    text: &str,
    asset_server: &Res<AssetServer>,
) -> Entity {
    return parent
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        })
        .id();
}
