use serde::{Deserialize, Serialize};
use tungstenite::{Message, Utf8Bytes};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessage {
    UpdatePlayerInfo {
        position: Option<(f32, f32)>,
        bullets: Vec<(f32, f32)>,
    },
    DamagedIntent {
        enemy_tag: u16,
    },
    DestroyEnemyIntent {
        bullet_tag: u16,
        enemy_tag: u16,
    },
}

impl ClientMessage {
    pub fn text(self) -> Message {
        Message::Text(Utf8Bytes::from(serde_json::to_string(&self).unwrap()))
    }
}
