use std::net::TcpStream;

use bevy::prelude::Component;
use shooting_game_shared::{ClientMessage, ServerMessage};
use tungstenite::{stream::MaybeTlsStream, Error, Message, WebSocket};

#[derive(Component)]
pub struct WebSocketClient(WebSocket<MaybeTlsStream<TcpStream>>);

impl WebSocketClient {
    pub fn new(websocket: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        Self(websocket)
    }

    pub fn read(&mut self) -> Result<Option<ServerMessage>, String> {
        match self.0.read() {
            Ok(message) => match message {
                Message::Text(text) => {
                    let message: ServerMessage = serde_json::from_str(&text).unwrap();
                    Ok(Some(message))
                }
                _ => Err("Invalid message type".to_string()),
            },
            Err(Error::Io(_)) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn send(&mut self, message: ClientMessage) -> Result<(), String> {
        match self.0.send(message.text()) {
            Ok(_) => Ok(()),
            Err(Error::Io(_)) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
