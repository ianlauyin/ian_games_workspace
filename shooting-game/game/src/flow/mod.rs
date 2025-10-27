mod game;
mod loading;
mod main_menu;
mod online_game;
mod shared;

use bevy::prelude::{App, Plugin};
pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game::AppGamePlugin,
            loading::AppLoadingPlugin,
            main_menu::MainMenuPlugin,
            shared::SharedSystemPlugin,
            online_game::OnlineGamePlugin,
        ));
    }
}
