mod asset_loader;
mod setup;

pub struct AppLoadingPlugin;

use bevy::prelude::{App, Plugin};
impl Plugin for AppLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((asset_loader::AssetLoaderPlugin, setup::SetupPlugin));
    }
}
