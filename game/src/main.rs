#![windows_subsystem = "windows"]

use std::time::Duration;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod components;
mod constant;
mod flow;
mod res;
mod states;
mod ui_components;
mod util;

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(components::ComponentPlugin)
        .add_plugins(flow::FlowPlugin)
        .add_plugins(res::ResPlugin)
        .add_plugins(states::StatePlugin)
        .add_plugins(ui_components::UIComponentsPlugin)
        .run();
}
