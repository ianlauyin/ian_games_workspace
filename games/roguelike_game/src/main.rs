use bevy::prelude::*;

mod flow;
mod startup;
mod state;

fn main() {
    App::new()
        .add_plugins(startup::StartupPlugin)
        .add_plugins(GameLibPlugin)
        .add_plugins(state::StatePlugin)
        .add_plugins(flow::FlowPlugin)
        .run();
}

pub struct GameLibPlugin;

impl Plugin for GameLibPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_lib::plugin::DevtoolPlugin,
            game_lib::component::OverlayNodePlugin,
        ));
    }
}
