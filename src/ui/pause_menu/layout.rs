use bevy::prelude::*;

use super::NORMAL_BUTTON_BG_COLOR;
use crate::ui::pause_menu::components::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            PauseMenu {},
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
                    build_button(parent, ResumeButton {}, "Resume", &asset_server);
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
