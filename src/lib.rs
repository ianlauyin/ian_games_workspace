use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use wasm_bindgen::prelude::wasm_bindgen;

use asset_loader::AssetPlugin;
use setup::SetupPlugin;
use ui::BackgroundPlugin;

use crate::control::ControlOptionPlugin;
use crate::ui::{ControlButtonPlugin, MainMenuPlugin, ResultPlugin};
use crate::util::VelocityPlugin;

mod asset_loader;
mod control;
mod game;
mod setup;
mod states;
mod ui;
mod util;

#[wasm_bindgen]
pub fn start() {
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
