#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use wasm_bindgen::prelude::wasm_bindgen;

use asset_loader::AssetPlugin;
use setup::SetupPlugin;
use ui::BackgroundPlugin;

use crate::ui::{MainMenuPlugin, ResultPlugin};
use crate::util::VelocityPlugin;

mod asset_loader;
mod game;
mod setup;
mod states;
mod ui;
mod util;

#[wasm_bindgen(start)]
pub fn main() {
    let ui_plugins = (
        AssetPlugin,
        SetupPlugin,
        BackgroundPlugin,
        MainMenuPlugin,
        ResultPlugin,
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
        .run();
}
