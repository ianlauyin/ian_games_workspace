use rocket::tokio::{sync::Mutex, time::sleep};
use std::{sync::Arc, time::Duration};

use crate::message::{Sender, ServerMessageHandler};

use super::{enemies::Enemies, players::Players};

pub type SharedGameState = Arc<Mutex<GameState>>;

#[derive(Default)]
enum Cycle {
    #[default]
    Matching,
    Ready,
    Playing,
}

#[derive(Default)]
pub struct GameState {
    cycle: Cycle,
    players: Players,
    enemies: Enemies,
    server_message_handler: ServerMessageHandler,
}

impl GameState {
    pub async fn check_cycle(&mut self) {
        match self.cycle {
            Cycle::Matching => {
                if self.players.matched().await {
                    self.server_message_handler.game_ready().await;
                    self.cycle = Cycle::Ready;
                }
            }
            Cycle::Ready => {
                if self.players.ready().await {
                    self.server_message_handler.game_start().await;
                    self.cycle = Cycle::Playing;
                }
            }
            _ => {}
        }
    }

    pub async fn new_player(&self, sender: Sender) -> u8 {
        let player_tag = self.players.new_player().await;
        self.server_message_handler
            .add_sender(player_tag, sender)
            .await;
        player_tag
    }

    pub async fn update_player_info(
        &self,
        player_tag: u8,
        position: (f32, f32),
        bullets: Vec<(f32, f32)>,
    ) {
        self.players
            .update_player_info(player_tag, position, bullets.clone())
            .await;
        self.server_message_handler
            .update_others_position(player_tag, position, bullets)
            .await;
    }
}
