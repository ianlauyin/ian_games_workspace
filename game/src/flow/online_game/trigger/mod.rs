mod update_position;

use bevy::prelude::*;

pub use update_position::*;
pub struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((update_position::UpdatePositionPlugin,));
    }
}
