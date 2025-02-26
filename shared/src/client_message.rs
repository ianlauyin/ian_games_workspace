use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessage {
    PlayerMove { x: f32, y: f32 },
}
