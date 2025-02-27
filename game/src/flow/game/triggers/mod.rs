mod remove_ufo;

pub use remove_ufo::RemoveUFOEvent;

use bevy::prelude::{App, Plugin};

pub struct TriggersPlugin;

impl Plugin for TriggersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((remove_ufo::RemoveUFOPlugin,));
    }
}
