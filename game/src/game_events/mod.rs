pub mod trigger;

use bevy::prelude::{App, Plugin};

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(trigger::TriggerPlugin);
    }
}
