use bevy::prelude::*;

use asset_loader::AssetPlugin;
pub use asset_loader::ImageHandles;
use setup::SetupPlugin;
use ui::BackgroundPlugin;

mod asset_loader;
mod constants;
mod game;
mod setup;
mod states;
mod ui;

fn main() {
    let ui_plugins = (AssetPlugin, SetupPlugin, BackgroundPlugin);

    let game_plugins = (
        game::VelocityPlugin,
        game::HealthPlugin,
        game::SpaceshipPlugin,
    );

    App::new()
        .add_plugins(ui_plugins)
        .add_plugins(game_plugins)
        .run();
}
