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

        self.send(player_tag, ServerMessage::Joined { player_tag })
            .await;
    }

    pub async fn game_ready(&self) {
        self.send_all(ServerMessage::GameReady).await;
    }

    pub async fn game_start(&self) {
        self.send_all(ServerMessage::GameStart).await;
    }

    pub async fn update_others_position(
        &self,
        player_tag: u8,
        position: (f32, f32),
        bullets: Vec<(f32, f32)>,
    ) {
        self.send_all_except(
            player_tag,
            ServerMessage::UpdatePosition {
                player_tag,
                position,
                bullets,
            },
        )
        .await;
    }

    async fn send(&self, tag: u8, message: ServerMessage) {
        let mut senders = self.0.lock().await;
        if let Some(sender) = senders.get_mut(&tag) {
            match sender.send(message.text()).await {
                Ok(_) => (),
                Err(_) => println!("Failed to send message to player {}", tag),
            }
        }
    }

    async fn send_all(&self, message: ServerMessage) {
        let senders = self.0.lock().await;
        let sender_tags: Vec<u8> = senders.keys().cloned().collect();
        drop(senders);

        for tag in sender_tags {
            self.send(tag, message.clone()).await;
        }
    }

    async fn send_all_except(&self, except_tag: u8, message: ServerMessage) {
        let senders = self.0.lock().await;
        let sender_tags: Vec<u8> = senders.keys().cloned().collect();
        drop(senders);

        for tag in sender_tags {
            if tag != except_tag {
                self.send(tag, message.clone()).await;
            }
        }
    }
}
