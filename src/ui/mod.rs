mod game_over_menu;
mod hud;
mod main_menu;
mod pause_menu;

use crate::ui::game_over_menu::GameOverMenuPlugin;
use crate::ui::hud::HUDPlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::pause_menu::PauseMenuPlugin;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HUDPlugin,
            MainMenuPlugin,
            PauseMenuPlugin,
            GameOverMenuPlugin,
        ));
    }
}
