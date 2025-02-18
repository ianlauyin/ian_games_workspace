mod velocity;
pub use velocity::Velocity;

use bevy::prelude::{App, Plugin};
pub struct UtilPlugin;
impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(velocity::VelocityPlugin);
    }
}
