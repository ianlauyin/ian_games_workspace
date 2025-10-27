#![windows_subsystem = "windows"]

use bevy::prelude::*;

mod components;
mod constant;
mod flow;
mod res;
mod stage;
mod states;
mod ui_components;
mod util;

fn main() {
    App::new()
        .add_plugins(flow::FlowPlugin)
        .add_plugins(games_lib::DevtoolPlugin)
        .add_plugins(components::ComponentPlugin)
        .add_plugins(res::ResPlugin)
        .add_plugins(states::StatePlugin)
        .add_plugins(ui_components::UIComponentsPlugin)
        .run();
}
