use rocket::tokio::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use crate::message::{Sender, ServerMessageHandler};

use super::{enemy::Enemy, player_info::PlayerInfo};

pub type SharedGameState = Arc<Mutex<GameState>>;

#[derive(Default)]
pub struct GameState {
    players: Mutex<HashMap<u8, PlayerInfo>>,
    enemies: Mutex<Vec<Enemy>>,
    message_handler: ServerMessageHandler,
}

impl GameState {
    pub async fn new_player(&self, sender: Sender) {
        let mut players = self.players.lock().await;
        let player_tag = players.len() as u8 + 1;
        let player = PlayerInfo::default();
        players.insert(player_tag, player);

        self.message_handler.add_sender(player_tag, sender).await;
    }
}
