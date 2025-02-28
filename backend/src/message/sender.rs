use rocket::{
    futures::{stream::SplitSink, SinkExt},
    tokio::{spawn, sync::RwLock},
};
use rocket_ws::{stream::DuplexStream, Message};
use shooting_game_shared::ServerMessage;
use std::{collections::HashMap, sync::Arc};

pub type Sender = SplitSink<DuplexStream, Message>;

#[derive(Default)]
pub struct ServerMessageHandler(RwLock<HashMap<u8, Arc<RwLock<Sender>>>>);

impl ServerMessageHandler {
    pub async fn add_sender(&self, player_tag: u8, sender: Sender) {
        let mut senders = self.0.write().await;
        senders.insert(player_tag, Arc::new(RwLock::new(sender)));
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

    pub async fn notice_others_position(
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

    pub async fn enemy_spawn(&self, tag: u16, position: (f32, f32), velocity: (f32, f32)) {
        self.send_all(ServerMessage::SpawnEnemy {
            tag,
            position,
            velocity,
        })
        .await;
    }

    pub async fn confirm_damaged(&self, player_tag: u8, enemy_tag: u16, health: u8) {
        self.send_all(ServerMessage::ConfirmDamaged {
            player_tag,
            enemy_tag,
            health,
        })
        .await;
    }

    pub async fn confirm_destroy_enemy(&self, player_tag: u8, bullet_tag: u16, enemy_tag: u16, new_score: u8) {
        self.send_all(ServerMessage::ConfirmDestroyEnemy {
            player_tag,
            bullet_tag,
            enemy_tag,
            new_score,
        })
        .await;
    }

    // Private
    async fn send(&self, tag: u8, message: ServerMessage) {
        let senders = self.0.read().await;
        if let Some(sender) = senders.get(&tag) {
            let sender_clone = sender.clone();
            spawn(async move {
                match sender_clone.write().await.send(message.text()).await {
                    Ok(_) => (),
                    Err(_) => println!("Failed to send message to player {}", tag),
                }
            });
        }
    }

    async fn send_all(&self, message: ServerMessage) {
        let senders = self.0.read().await;
        let sender_tags: Vec<u8> = senders.keys().cloned().collect();
        drop(senders);

        for tag in sender_tags {
            self.send(tag, message.clone()).await;
        }
    }

    async fn send_all_except(&self, except_tag: u8, message: ServerMessage) {
        let senders = self.0.read().await;
        let sender_tags: Vec<u8> = senders.keys().cloned().collect();
        drop(senders);

        for tag in sender_tags {
            if tag != except_tag {
                self.send(tag, message.clone()).await;
            }
        }
    }
}
