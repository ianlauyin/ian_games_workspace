use serde::{Deserialize, Serialize};
use tungstenite::{Message, Utf8Bytes};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessage {
    UpdatePlayerInfo {
        position: (f32, f32),
        bullets: Vec<(f32, f32)>,
    },
}

impl ClientMessage {
    pub fn text(self) -> Message {
        Message::Text(Utf8Bytes::from(serde_json::to_string(&self).unwrap()))
    }
}
