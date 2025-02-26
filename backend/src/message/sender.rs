use rocket::{
    futures::{stream::SplitSink, SinkExt},
    tokio::sync::Mutex,
};
use rocket_ws::{stream::DuplexStream, Message};
use shooting_game_shared::ServerMessage;
use std::collections::HashMap;

pub type Sender = SplitSink<DuplexStream, Message>;

#[derive(Default)]
pub struct ServerMessageHandler(Mutex<HashMap<u8, Sender>>);

impl ServerMessageHandler {
    pub async fn add_sender(&self, player_tag: u8, sender: Sender) {
        let mut senders = self.0.lock().await;
        senders.insert(player_tag, sender);
        drop(senders);

        self.send_to(player_tag, ServerMessage::Joined { player_tag })
            .await;
    }

    async fn send_to(&self, tag: u8, message: ServerMessage) {
        let mut senders = self.0.lock().await;
        if let Some(sender) = senders.get_mut(&tag) {
            match sender.send(message.text()).await {
                Ok(_) => (),
                Err(_) => println!("Failed to send message to player {}", tag),
            }
        }
    }
}
