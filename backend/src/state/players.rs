use std::collections::HashMap;

use rocket::tokio::sync::Mutex;

#[derive(Default)]
pub struct Players(Mutex<HashMap<u8, PlayerInfo>>);

impl Players {
    pub async fn new_player(&self) -> u8 {
        let mut players = self.0.lock().await;
        let player_tag = players.len() as u8 + 1;
        let player = PlayerInfo::default();
        players.insert(player_tag, player);
        player_tag
    }
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
