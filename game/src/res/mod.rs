mod control_option;
mod image_handles;

pub use control_option::{ControlMode, ControlOption};
pub use image_handles::ImageHandles;

use bevy::prelude::{App, Plugin};
pub struct ResPlugin;
impl Plugin for ResPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageHandles>()
            .insert_resource(ControlOption {
                mode: ControlMode::Keyboard,
            });
    }
}
