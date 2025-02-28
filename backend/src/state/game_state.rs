use rocket::tokio::sync::RwLock;
use rocket_ws::result::Error;
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
}

#[derive(Default)]
pub struct GameState {
    cycle: Cycle,
    players: Players,
    stage: RwLock<Stage>,
    enemies: RwLock<Vec<u16>>,
    server_message_handler: ServerMessageHandler,
}

impl GameState {
    pub async fn new_player(&mut self, sender: Sender) -> u8 {
        let player_tag = self.players.new_player().await;
        self.server_message_handler
            .add_sender(player_tag, sender)
            .await;
        player_tag
    }

    pub async fn update_player_info(
        &self,
        player_tag: u8,
        position: Option<(f32, f32)>,
        bullets: Vec<(f32, f32)>,
    ) {
        self.players
            .update_player_info(player_tag, position, bullets.clone())
            .await;
    }

    pub async fn player_damaged(&mut self, player_tag: u8, enemy_tag: u16) {
        let mut enemies = self.enemies.write().await;
        if enemies.contains(&enemy_tag) {
            let health = self.players.damaged(player_tag).await;
            let _ = self
                .server_message_handler
                .confirm_damaged(player_tag, enemy_tag, health)
                .await;
            enemies.retain(|&tag| tag != enemy_tag);
            drop(enemies);
            self.check_game_over().await;
        }
    }

    pub async fn destroy_enemy(&mut self, player_tag: u8, bullet_tag: u16, enemy_tag: u16) {
        let mut enemies = self.enemies.write().await;
        if enemies.contains(&enemy_tag) {
            let new_score = self.players.add_score(player_tag).await;
            let _ = self
                .server_message_handler
                .confirm_destroy_enemy(player_tag, bullet_tag, enemy_tag, new_score)
                .await;
            enemies.retain(|&tag| tag != enemy_tag);
            drop(enemies);
            self.update_stage().await;
        }
    }

    // Private
    async fn notice_player_info(&mut self) -> Result<(), Error> {
        let players = self.players.get_players_info().await;
        for (player_tag, position, bullets) in players {
            self.server_message_handler
                .notice_others_position(player_tag, position, bullets)
                .await?;
        }
        Ok(())
    }

    async fn spawn_enemy(&mut self) -> Result<(), Error> {
        let mut enemies = self.enemies.write().await;
        let stage = self.stage.read().await;
        let ufo_numbers = enemies.len() + 1;
        if !stage.random_generator(ufo_numbers) {
            return Ok(());
        }
        let tag = UFORandomGenerator::tag();
        let position = UFORandomGenerator::position();
        let velocity = stage.get_ufo_velocity_tuple();
        if enemies.contains(&tag) {
            return Ok(());
        }
        enemies.push(tag);
        self.server_message_handler
            .enemy_spawn(tag, position, velocity)
            .await
    }

    async fn check_game_over(&mut self) {
        if self.players.all_players_dead().await {
            self.server_message_handler.game_over().await;
            self.cleanup().await;
        }
    }

    async fn cleanup(&mut self) {
        self.enemies.write().await.clear();
        self.players.clear_players().await;
        *self.stage.write().await = Stage::default();
        self.server_message_handler.clear_senders().await;
        self.cycle = Cycle::Matching;
    }

    async fn update_stage(&self) {
        let total_score = self.players.get_total_score().await;
        let new_stage = Stage::new(total_score);
        let mut stage = self.stage.write().await;
        *stage = new_stage;
    }

    // Cycle Related (Not run in the main thread)
    pub async fn check_cycle(&mut self) -> Cycle {
        match self.cycle {
            Cycle::Matching => self.handle_cycle_matching().await,
            Cycle::Ready => self.handle_cycle_ready().await,
            Cycle::Playing => self.handle_cycle_playing().await,
        }
        self.cycle.clone()
    }

    async fn handle_cycle_matching(&mut self) {
        if self.players.matched().await {
            let _ = self.server_message_handler.game_ready().await;
            self.cycle = Cycle::Ready;
        }
    }

    async fn handle_cycle_ready(&mut self) {
        let _ = self.notice_player_info().await;
        if self.players.ready().await {
            let _ = self.server_message_handler.game_start().await;
            self.cycle = Cycle::Playing;
        }
    }

    async fn handle_cycle_playing(&mut self) {
        let _ = self.notice_player_info().await;
        let _ = self.spawn_enemy().await;
    }
}
