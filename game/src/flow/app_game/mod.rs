mod cleanup;
mod in_play;
mod ready;
mod triggers;

use bevy::prelude::{App, Plugin};
pub struct AppGamePlugin;
impl Plugin for AppGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ready::ReadyPlugin,
            in_play::InPlayPlugin,
            triggers::TriggersPlugin,
            cleanup::CleanupPlugin,
        ));
    }
}
