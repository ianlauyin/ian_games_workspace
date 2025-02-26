use crate::state::SharedGameState;
use rocket::futures::stream::SplitStream;
use rocket::futures::StreamExt;
use rocket_ws::stream::DuplexStream;
use shooting_game_shared::ClientMessage;

pub type Receiver = SplitStream<DuplexStream>;

pub struct ClientMessageHandler(SharedGameState);

impl ClientMessageHandler {
    pub fn new(game_state: SharedGameState) -> Self {
        Self(game_state)
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
        println!("Received message: {:?}", message);
    }
}
