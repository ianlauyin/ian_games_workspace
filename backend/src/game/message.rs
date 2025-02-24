use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Join,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Joined { player_tag: u8 },
}
