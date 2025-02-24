use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};

pub static GAME_STATE: LazyLock<GameState> = LazyLock::new(|| GameState::default());

#[derive(Default)]
pub struct GameState {
    players: Mutex<Vec<PlayerInfo>>,
    enemies: Vec<Enemy>,
}

impl GameState {
    pub fn new_player(&self) -> u8 {
        let mut players = self.players.lock().unwrap();
        let player_tag = players.len() as u8 + 1;
        players.push(PlayerInfo {
            player_tag,
            health: 3,
            ..Default::default()
        });
        player_tag
    }
}

pub struct PlayerInfo {
    pub player_tag: u8,
    pub score: u16,
    pub health: u16,
    pub position: (f32, f32),
    pub bullets: Vec<(f32, f32)>,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            player_tag: 0,
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
