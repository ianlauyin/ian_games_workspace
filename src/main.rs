use bevy::prelude::*;

pub use asset_loader::AssetHandles;
use asset_loader::AssetPlugin;

use crate::setup::SetupPlugin;
use crate::ui::BackgroundPlugin;

mod asset_loader;
mod setup;
mod states;
mod ui;

fn main() {
    let custom_plugins = (AssetPlugin, SetupPlugin, BackgroundPlugin);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(custom_plugins)
        .run();
}
