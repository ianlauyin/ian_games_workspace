use bevy::prelude::*;

use super::websocket_client::WebSocketClient;

pub struct SendMessagePlugin;

impl Plugin for SendMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_message);
    }
}

fn send_message(mut web_socket_clients: Query<&WebSocketClient>) {}
