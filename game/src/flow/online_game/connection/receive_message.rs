use bevy::prelude::*;
use shooting_game_shared::ServerMessage;

use crate::states::AppState;

use super::websocket_client::WebSocketClient;

#[derive(Event)]
pub struct ReceiveMessageEvent(pub ServerMessage);

pub struct ReceiveMessagePlugin;

impl Plugin for ReceiveMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            receive_message.run_if(in_state(AppState::OnlineGame)),
        );
    }
}

fn receive_message(mut commands: Commands, mut web_socket_clients: Query<&mut WebSocketClient>) {
    for mut client in web_socket_clients.iter_mut() {
        match client.read() {
            Ok(Some(message)) => commands.trigger(ReceiveMessageEvent(message)),
            Ok(None) => {}
            Err(e) => warn!("error receiving: {e}"),
        }
    }
}
