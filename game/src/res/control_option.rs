use bevy::prelude::Resource;

#[derive(Eq, PartialEq)]
pub enum ControlMode {
    Keyboard,
    Button,
}

#[derive(Resource)]
pub struct ControlOption {
    pub mode: ControlMode,
}

impl ControlOption {
    pub fn toggle(&mut self) {
        self.mode = match self.mode {
            ControlMode::Keyboard => ControlMode::Button,
            ControlMode::Button => ControlMode::Keyboard,
        };
    }
}
