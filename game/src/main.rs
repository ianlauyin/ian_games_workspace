#![windows_subsystem = "windows"]

use bevy::prelude::App;
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod app_loading;
mod app_menu;
mod constant;
mod app_game;
mod game_component;
mod res;
mod shared_state_system;
mod states;
mod ui_component;
mod util;

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(app_loading::AppLoadingPlugin)
        .add_plugins(app_menu::AppMenuPlugin)
        .add_plugins(app_game::GamePlugin)
        .add_plugins(game_component::GameComponentPlugin)
        .add_plugins(res::ResPlugin)
        .add_plugins(shared_state_system::SharedSystemPlugin)
        .add_plugins(states::StatePlugin)
        .add_plugins(ui_component::UIComponentPlugin)
        .add_plugins(util::UtilPlugin)
        .run();
}
