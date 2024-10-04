use bevy::prelude::*;

use crate::asset_loader::AssetPlugin;

mod asset_loader;
mod states;
mod ui;

fn main() {
    let custom_plugins = (AssetPlugin);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(custom_plugins)
        .run();
}
