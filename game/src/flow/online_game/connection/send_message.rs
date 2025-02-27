use bevy::prelude::*;
use shooting_game_shared::ClientMessage;

use crate::states::AppState;

use super::websocket_client::WebSocketClient;

#[derive(Event)]
pub struct SendMessageEvent(pub ClientMessage);
pub struct SendMessagePlugin;

impl Plugin for SendMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(send_message);
    }
}

fn send_message(
    trigger: Trigger<SendMessageEvent>,
    current_state: Res<State<AppState>>,
    mut web_socket_clients: Query<&mut WebSocketClient>,
) {
    if *current_state.get() != AppState::OnlineGame {
        return;
    }
    if web_socket_clients.is_empty() {
        return;
    };
    let Ok(mut client) = web_socket_clients.get_single_mut() else {
        warn!("Should only have one websocket client");
        return;
    };
    let client_message = trigger.event().0.clone();
    if let Err(e) = client.send(client_message) {
        warn!("Error sending message: {e}");
    }
}
