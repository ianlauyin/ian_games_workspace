use bevy::prelude::*;

mod flow;
mod startup;
mod state;

fn main() {
    App::new()
        .add_plugins(startup::StartupPlugin)
        .add_plugins(game_lib::plugin::DevtoolPlugin)
        .add_plugins(state::StatePlugin)
        .add_plugins(flow::FlowPlugin)
        .run();
}
