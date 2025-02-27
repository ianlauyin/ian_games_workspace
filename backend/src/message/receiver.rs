use crate::state::SharedGameState;
use rocket::futures::stream::SplitStream;
use rocket::futures::StreamExt;
use rocket_ws::stream::DuplexStream;
use shooting_game_shared::ClientMessage;

pub type Receiver = SplitStream<DuplexStream>;

pub struct ClientMessageHandler {
    player_tag: u8,
    shared_game_state: SharedGameState,
}

impl ClientMessageHandler {
    pub fn new(player_tag: u8, shared_game_state: SharedGameState) -> Self {
        Self {
            player_tag,
            shared_game_state,
        }
    }

    pub async fn handle_messages(&self, mut receiver: Receiver) {
        while let Some(message) = receiver.next().await {
            if let Ok(msg) = message {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&msg.to_string()) {
                    self.handle_message(client_msg).await;
                }
            }
        }
    }

    async fn handle_message(&self, message: ClientMessage) {
        let game_state = self.shared_game_state.lock().await;
        match message {
            ClientMessage::UpdatePlayerInfo { position, bullets } => {
                game_state
                    .update_player_info(self.player_tag, position, bullets)
                    .await
            }
        }
    }
}
