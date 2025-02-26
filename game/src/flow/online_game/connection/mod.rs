mod handler;
mod receive_message;
mod send_message;
mod websocket_client;

pub use receive_message::ReceiveMessageEvent;
pub use send_message::SendMessageEvent;

use bevy::prelude::*;

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            handler::HandlerPlugin,
            receive_message::ReceiveMessagePlugin,
            send_message::SendMessagePlugin,
        ));
    }
}
