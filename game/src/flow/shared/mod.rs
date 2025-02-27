mod cleanup;
mod control;
pub mod game_trigger;
mod shooting;
mod stars;

use bevy::prelude::{App, Plugin};
pub struct SharedSystemPlugin;

impl Plugin for SharedSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            stars::StarsPlugin,
            cleanup::CleanupPlugin,
            game_trigger::GameTriggerPlugin,
            control::ControlPlugin,
            shooting::ShootingPlugin,
        ));
    }
}
