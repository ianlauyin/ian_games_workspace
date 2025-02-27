use rocket::tokio::sync::RwLock;
use shooting_game_shared::game_related::{Stage, UFORandomGenerator};
use std::sync::Arc;

use crate::message::{Sender, ServerMessageHandler};

use super::players::Players;

pub type SharedGameState = Arc<RwLock<GameState>>;

#[derive(Default, Clone)]
pub enum Cycle {
    #[default]
    Matching,
    Ready,
    Playing,
    Cleanup,
}

#[derive(Default)]
pub struct GameState {
    cycle: Cycle,
    players: Players,
    stage: RwLock<Stage>,
    enemies: RwLock<Vec<u128>>,
    server_message_handler: ServerMessageHandler,
}

impl GameState {
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
    }

    async fn notice_player_info(&mut self) {
        let players = self.players.get_players_info().await;
        for (player_tag, position, bullets) in players {
            self.server_message_handler
                .notice_others_position(player_tag, position, bullets)
                .await;
        }
    }

    async fn spawn_enemy(&mut self) {
        let mut enemies = self.enemies.write().await;
        let stage = self.stage.read().await;
        let ufo_numbers = enemies.len() + 1;
        if !stage.random_generator(ufo_numbers) {
            return;
        }
        let tag = UFORandomGenerator::tag();
        let position = UFORandomGenerator::position();
        let velocity = stage.get_ufo_velocity_tuple();
        if !enemies.contains(&tag) {
            enemies.push(tag);
        }
        self.server_message_handler
            .enemy_spawn(tag, position, velocity)
            .await;
    }

    // Cycle Related (Not run in the main thread)
    pub async fn check_cycle(&mut self) -> Cycle {
        match self.cycle {
            Cycle::Matching => self.handle_cycle_matching().await,
            Cycle::Ready => self.handle_cycle_ready().await,
            Cycle::Playing => self.handle_cycle_playing().await,
            Cycle::Cleanup => self.handle_cycle_cleanup().await,
        }
        self.cycle.clone()
    }

    async fn handle_cycle_matching(&mut self) {
        if self.players.matched().await {
            self.server_message_handler.game_ready().await;
            self.cycle = Cycle::Ready;
        }
    }

    async fn handle_cycle_ready(&mut self) {
        self.notice_player_info().await;
        if self.players.ready().await {
            self.server_message_handler.game_start().await;
            self.cycle = Cycle::Playing;
        }
    }

    async fn handle_cycle_playing(&mut self) {
        self.notice_player_info().await;
        self.spawn_enemy().await;
    }

    async fn handle_cycle_cleanup(&mut self) {
        self.cycle = Cycle::Matching;
    }
}
