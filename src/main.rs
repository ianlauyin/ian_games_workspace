use bevy::prelude::*;

use asset_loader::AssetPlugin;
use setup::SetupPlugin;
use ui::BackgroundPlugin;

use crate::ui::MainMenuPlugin;
use crate::util::VelocityPlugin;

mod asset_loader;
mod game;
mod setup;
mod states;
mod ui;
mod util;

fn main() {
    let ui_plugins = (AssetPlugin, SetupPlugin, BackgroundPlugin, MainMenuPlugin);

    let game_plugins = (
        game::HealthPlugin,
        game::SpaceshipPlugin,
        game::BulletPlugin,
        game::UFOPlugin,
        game::ScorePlugin,
        game::CollisionPlugin,
        game::ExplosionPlugin,
        game::InvisiblePlugin,
    );

    let util_plugins = VelocityPlugin;

    App::new()
        .add_plugins(ui_plugins)
        .add_plugins(game_plugins)
        .add_plugins(util_plugins)
        .run();
}
