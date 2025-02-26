use rocket::tokio::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use crate::message::{Sender, ServerMessageHandler};

use super::{enemies::Enemies, players::Players};

pub type SharedGameState = Arc<Mutex<GameState>>;

#[derive(Default)]
pub struct GameState {
    players: Players,
    enemies: Enemies,
    server_message_handler: ServerMessageHandler,
}

impl GameState {
    pub async fn new_player(&self, sender: Sender) {
        let player_tag = self.players.new_player().await;
        self.server_message_handler
            .add_sender(player_tag, sender)
            .await;
    }
}
