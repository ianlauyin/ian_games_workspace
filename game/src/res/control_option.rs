use bevy::prelude::{Component, Resource};

#[derive(Component, Clone, Eq, PartialEq)]
pub enum ControlMode {
    Keyboard,
    Button,
}

#[derive(Resource)]
pub struct ControlOption {
    pub mode: ControlMode,
}

impl ControlOption {
    pub fn set_mode(&mut self, mode: &ControlMode) {
        self.mode = mode.clone();
    }
}
