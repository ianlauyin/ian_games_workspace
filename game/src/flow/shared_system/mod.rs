mod cleanup;
mod stars;

use bevy::prelude::{App, Plugin};
pub struct SharedSystemPlugin;

impl Plugin for SharedSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((stars::StarsPlugin, cleanup::CleanupPlugin));
    }
}
