mod main_menu;

use bevy::prelude::*;

pub struct AppMenuPlugin;

impl Plugin for AppMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(main_menu::MainMenuPlugin);
    }
}
