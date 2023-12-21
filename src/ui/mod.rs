pub mod hud;
pub mod main_menu;
pub mod pause_menu;

use crate::ui::hud::HUDPlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::pause_menu::PauseMenuPlugin;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HUDPlugin, MainMenuPlugin, PauseMenuPlugin));
    }
}
