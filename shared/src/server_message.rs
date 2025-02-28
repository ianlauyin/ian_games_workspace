use rocket_ws::Message;
use serde::{Deserialize, Serialize};

pub type Position = (f32, f32);
pub type Velocity = (f32, f32);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServerMessage {
    Joined {
        player_tag: u8,
    },
    GameReady,
    GameStart,
    UpdatePosition {
        player_tag: u8,
        position: Position,
        bullets: Vec<Position>,
    },
    SpawnEnemy {
        tag: u16,
        position: Position,
        velocity: Velocity,
    },
    ConfirmDamaged {
        player_tag: u8,
        enemy_tag: u16,
        health: u8,
    },
    ConfirmDestroyEnemy {
        player_tag: u8,
        bullet_tag: u16,
        enemy_tag: u16,
        new_score: u8,
    },
}

impl ServerMessage {
    pub fn text(self) -> Message {
        Message::Text(serde_json::to_string(&self).unwrap())
    }
}
