#![windows_subsystem = "windows"]

use bevy::prelude::App;
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod constant;
mod flow;
mod game_component;
mod game_events;
mod res;
mod states;
mod ui_component;
mod util;

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(flow::FlowPlugin)
        .add_plugins(game_component::GameComponentPlugin)
        .add_plugins(game_events::GameEventsPlugin)
        .add_plugins(res::ResPlugin)
        .add_plugins(states::StatePlugin)
        .add_plugins(ui_component::UIComponentPlugin)
        .run();
}
