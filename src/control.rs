use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

#[derive(Eq, PartialEq)]
pub enum ControlMode {
    Keyboard,
    Hover,
}

#[derive(Resource)]
pub struct ControlOption {
    pub mode: ControlMode,
}

pub struct ControlOptionPlugin;

impl Plugin for ControlOptionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlOption {
            mode: ControlMode::Keyboard,
        });
    }
}
