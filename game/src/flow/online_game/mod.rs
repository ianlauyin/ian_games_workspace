mod connection;
mod matching;

use bevy::prelude::*;

pub struct OnlineGamePlugin;

impl Plugin for OnlineGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((connection::ConnectionPlugin, matching::MatchingPlugin));
    }
}
