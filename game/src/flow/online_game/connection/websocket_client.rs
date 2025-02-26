use std::net::TcpStream;

use bevy::{log::warn, prelude::Component};
use shooting_game_shared::ServerMessage;
use tungstenite::{http::Response, stream::MaybeTlsStream, Error, Message, WebSocket};

#[derive(Component)]
pub struct WebSocketClient {
    websocket: WebSocket<MaybeTlsStream<TcpStream>>,
    response: Response<Option<Vec<u8>>>,
}

impl WebSocketClient {
    pub fn new(
        client: (
            WebSocket<MaybeTlsStream<TcpStream>>,
            Response<Option<Vec<u8>>>,
        ),
    ) -> Self {
        Self {
            websocket: client.0,
            response: client.1,
        }
    }

    pub fn read(&mut self) -> Result<Option<ServerMessage>, String> {
        match self.websocket.read() {
            Ok(message) => match message {
                Message::Text(text) => {
                    let message: ServerMessage = serde_json::from_str(&text).unwrap();
                    Ok(Some(message))
                }
                _ => Err("Invalid message type".to_string()),
            },
            Err(tungstenite::Error::Io(_)) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
}
