use rocket::futures::stream::SplitStream;
use rocket_ws::stream::DuplexStream;
use serde::{Deserialize, Serialize};

pub type Receiver = SplitStream<DuplexStream>;

#[derive(Default)]
pub struct ClientMessageHandler;

#[derive(Serialize, Deserialize, Clone)]
enum ClientMessage {
    Join,
}
