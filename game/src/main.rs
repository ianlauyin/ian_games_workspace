#![windows_subsystem = "windows"]

use asset_loader::AssetPlugin;
use bevy::app::App;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use control::ControlOptionPlugin;
use setup::SetupPlugin;
use ui::{BackgroundPlugin, ControlButtonPlugin, MainMenuPlugin, ResultPlugin};
use util::VelocityPlugin;

mod asset_loader;
mod control;
mod game;
mod setup;
mod states;
mod ui;
mod util;

fn main() {
    let ui_plugins = (
        AssetPlugin,
        SetupPlugin,
        BackgroundPlugin,
        MainMenuPlugin,
        ResultPlugin,
        ControlButtonPlugin,
    );

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
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(ui_plugins)
        .add_plugins(game_plugins)
        .add_plugins(util_plugins)
        .add_plugins(ControlOptionPlugin)
        .run();
}
