mod connection;
mod in_play;
mod matching;
mod ready;
mod result;
mod shared;
mod trigger;

use bevy::prelude::*;

pub struct OnlineGamePlugin;

impl Plugin for OnlineGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            connection::ConnectionPlugin,
            matching::MatchingPlugin,
            ready::ReadyPlugin,
            shared::SharedPlugin,
            trigger::TriggerPlugin,
            in_play::InPlayPlugin,
            result::ResultPlugin,
        ));
    }
}
