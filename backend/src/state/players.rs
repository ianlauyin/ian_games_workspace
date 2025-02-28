use std::collections::HashMap;

use rocket::tokio::sync::RwLock;
use shooting_game_shared::util::EdgeUtil;

#[derive(Default)]
pub struct Players(RwLock<HashMap<u8, PlayerInfo>>);

impl Players {
    pub async fn new_player(&self) -> u8 {
        let mut players = self.0.write().await;
        let mut player_tag = 1;
        while players.contains_key(&player_tag) {
            player_tag += 1;
        }
        let player = PlayerInfo::default();
        players.insert(player_tag, player);
        player_tag
    }

    pub async fn all_players_dead(&self) -> bool {
        let players = self.0.read().await;
        players.values().all(|player| player.health == 0)
    }

    pub async fn get_total_score(&self) -> u8 {
        let players = self.0.read().await;
        players.values().map(|player| player.score).sum()
    }

    pub async fn get_players_info(&self) -> Vec<(u8, (f32, f32), Vec<(f32, f32)>)> {
        self.0
            .read()
            .await
            .iter()
            .map(|(tag, player)| (*tag, player.position, player.bullets.clone()))
            .collect()
    }

    pub async fn matched(&self) -> bool {
        let players = self.0.read().await;
        players.len() == 2
    }

    pub async fn ready(&self) -> bool {
        let players = self.0.read().await;
        let mut ready_count = 0;
        for player in players.values() {
            let edge_util = EdgeUtil::spaceship();
            if !edge_util.over_bottom_in(player.position.1) {
                ready_count += 1;
            }
        }
        ready_count == players.len()
    }

    pub async fn update_player_info(
        &self,
        player_tag: u8,
        position: Option<(f32, f32)>,
        bullets: Vec<(f32, f32)>,
    ) {
        let mut players = self.0.write().await;
        players.entry(player_tag).and_modify(|player| {
            if let Some(position) = position {
                player.position = position;
            }
            player.bullets = bullets;
        });
    }

    pub async fn damaged(&self, player_tag: u8) -> u8 {
        let mut players = self.0.write().await;
        let player = players.get_mut(&player_tag).unwrap();
        player.health -= 1;
        player.health
    }

    pub async fn add_score(&self, player_tag: u8) -> u8 {
        let mut players = self.0.write().await;
        let player = players.get_mut(&player_tag).unwrap();
        player.score += 1;
        player.score
    }

    pub async fn clear_players(&self) {
        let mut players = self.0.write().await;
        players.clear();
    }
}

#[derive(Debug)]
struct PlayerInfo {
    score: u8,
    health: u8,
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
