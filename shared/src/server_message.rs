use rocket_ws::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerMessage {
    Joined { player_tag: u8 },
    StartGame,
}

impl ServerMessage {
    pub fn text(self) -> Message {
        Message::Text(serde_json::to_string(&self).unwrap())
    }
}
