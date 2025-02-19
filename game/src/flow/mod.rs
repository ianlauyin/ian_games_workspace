mod app_game;
mod app_loading;
mod app_menu;
mod shared_system;

use bevy::prelude::{App, Plugin};
pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            app_game::AppGamePlugin,
            app_loading::AppLoadingPlugin,
            app_menu::AppMenuPlugin,
            shared_system::SharedSystemPlugin,
        ));
    }
}
