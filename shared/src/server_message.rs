use rocket_ws::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServerMessage {
    Joined {
        player_tag: u8,
    },
    GameReady,
    GameStart,
    UpdatePosition {
        player_tag: u8,
        position: (f32, f32),
        bullets: Vec<(f32, f32)>,
    },
}

impl ServerMessage {
    pub fn text(self) -> Message {
        Message::Text(serde_json::to_string(&self).unwrap())
    }
}
