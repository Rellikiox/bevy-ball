use bevy::prelude::*;

use crate::ui::hud::components::*;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            build_tracker(parent, &asset_server, StarTracker {}, "sprites/star.png");
            build_tracker(
                parent,
                &asset_server,
                EnemyTracker {},
                "sprites/ball_red_large.png",
            );
        });
}

pub fn build_tracker<T: Component>(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    marker: T,
    icon_path: &str,
) -> Entity {
    return parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,

                row_gap: Val::Px(16.0),
                ..default()
            },
            background_color: Color::rgb(0.7, 0.7, 0.7).into(),
            ..default()
        })
        .with_children(|parent| {
            build_image(parent, &asset_server, icon_path);
            build_text(parent, &asset_server, marker, "0");
        })
        .id();
}

pub fn build_image(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    image_path: &str,
) -> Entity {
    return parent
        .spawn(ImageBundle {
            style: Style {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
                ..default()
            },
            image: asset_server.load(image_path.to_string()).into(),
            ..default()
        })
        .id();
}

pub fn build_text<T: Component>(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    marker: T,
    text: &str,
) -> Entity {
    return parent
        .spawn((
            TextBundle {
                style: Style {
                    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
                    ..default()
                },
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
            },
            marker,
        ))
        .id();
}
