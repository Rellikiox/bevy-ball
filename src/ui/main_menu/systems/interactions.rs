use crate::ui::main_menu::{components::*, styles::*};
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn interact_with_play_button(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut bg_color)) = query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_BG_COLOR.into();
                next_app_state.set(AppState::Game);
            }
            Interaction::Hovered => *bg_color = HOVER_BUTTON_BG_COLOR.into(),
            Interaction::None => *bg_color = NORMAL_BUTTON_BG_COLOR.into(),
        };
    }
}

pub fn interact_with_quit_button(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_ew: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut bg_color)) = query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_BG_COLOR.into();
                app_exit_ew.send(AppExit);
            }
            Interaction::Hovered => *bg_color = HOVER_BUTTON_BG_COLOR.into(),
            Interaction::None => *bg_color = NORMAL_BUTTON_BG_COLOR.into(),
        }
    }
}
