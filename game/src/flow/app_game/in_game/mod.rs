mod control_panel;

use bevy::prelude::*;
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(control_panel::ControlPanelPlugin);
    }
}
