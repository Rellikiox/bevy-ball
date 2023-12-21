use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            build_title(parent, asset_server);
            build_button(parent, PlayButton {}, "Play", asset_server);
            build_button(parent, PlayButton {}, "Quit", asset_server);
        })
        .id();

    return main_menu_entity;
}

pub fn build_title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) -> Entity {
    return parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(300.0),
                height: Val::Px(120.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            build_image(parent, asset_server, "sprites/ball_blue_large.png");
            build_text(parent, "Ballz", asset_server);
            build_image(parent, asset_server, "sprites/ball_red_large.png");
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
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
                ..default()
            },
            image: asset_server.load(image_path).into(),
            ..default()
        })
        .id();
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
                background_color: NORMAL_BUTTON_BG_COLOR.into(),
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
