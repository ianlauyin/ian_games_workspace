use futures::stream::{SplitSink, SplitStream};
use rocket_ws::{stream::DuplexStream, Message};
use serde::{Deserialize, Serialize};

pub type Receiver = SplitStream<DuplexStream>;
pub type Sender = SplitSink<DuplexStream, Message>;

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientMessage {
    Join,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerMessage {
    Joined { player_tag: u8 },
}
