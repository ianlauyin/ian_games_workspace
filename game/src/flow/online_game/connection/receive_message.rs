use bevy::prelude::*;

use super::websocket_client::WebSocketClient;

pub struct ReceiveMessagePlugin;

impl Plugin for ReceiveMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, receive_message);
    }
}

fn receive_message(mut web_socket_clients: Query<&mut WebSocketClient>) {
    for mut client in web_socket_clients.iter_mut() {
        match client.read() {
            Ok(Some(message)) => info!("Received message {message:?}"),
            Ok(None) => {}
            Err(e) => warn!("error receiving: {e}"),
        }
    }
}
