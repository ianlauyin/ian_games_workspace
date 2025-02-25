use std::{collections::HashMap, sync::Arc};

use futures::SinkExt;
use rocket::tokio::sync::Mutex;
use rocket_ws::Message;

use super::message::Sender;

pub type ArcGameState = Arc<Mutex<GameState>>;

pub struct GameState {
    players: Mutex<HashMap<u8, Player>>,
    enemies: Vec<Enemy>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: Mutex::new(HashMap::new()),
            enemies: Vec::new(),
        }
    }

    pub async fn new_player(&self, sender: Arc<Mutex<Sender>>) {
        let mut players = self.players.lock().await;
        let player_tag = players.len() as u8 + 1;
        let player = Player {
            info: PlayerInfo::default(),
            sender: sender.clone(),
        };
        players.insert(player_tag, player);
        sender
            .lock()
            .await
            .send(Message::text(format!("Joined {player_tag}")))
            .await
            .unwrap();
    }
}

struct Player {
    info: PlayerInfo,
    sender: Arc<Mutex<Sender>>,
}

struct PlayerInfo {
    score: u16,
    health: u16,
    position: (f32, f32),
    bullets: Vec<(f32, f32)>,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            score: 0,
            health: 3,
            position: (0.0, 0.0),
            bullets: Vec::new(),
        }
    }
}

pub struct Enemy {
    pub tag: u8,
    pub position: (f32, f32),
}
