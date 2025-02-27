mod control_option;
mod image_handles;
mod player_tag;

use bevy::prelude::{App, Plugin};
pub use control_option::{ControlMode, ControlOption};
pub use image_handles::ImageHandles;
pub use player_tag::PlayerTag;
pub struct ResPlugin;
impl Plugin for ResPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageHandles>()
            .insert_resource(ControlOption {
                mode: ControlMode::Keyboard,
            })
            .insert_resource(PlayerTag(1));
    }
}
