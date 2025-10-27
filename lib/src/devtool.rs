use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DevtoolPlugin;

impl Plugin for DevtoolPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }
}
