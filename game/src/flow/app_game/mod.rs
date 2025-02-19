mod in_game;

use bevy::prelude::{App, Plugin};
pub struct AppGamePlugin;
impl Plugin for AppGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(in_game::InGamePlugin);
    }
}
