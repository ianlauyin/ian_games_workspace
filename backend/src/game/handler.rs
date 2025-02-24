use rocket_ws::{Message, Stream, WebSocket};
use serde_json;

use super::message::{ClientMessage, ServerMessage};
use super::state::GAME_STATE;

#[rocket::get("/game")]
pub fn ws_handler(ws: WebSocket) -> Stream!['static] {
    Stream! {
        ws => {
            for await message in ws {
                match message {
                    Ok(Message::Text(text)) => {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                            match client_msg {
                                ClientMessage::Join => {
                                    let tag = GAME_STATE.new_player();
                                    let response = ServerMessage::Joined { player_tag: tag };
                                    yield Message::Text(serde_json::to_string(&response).unwrap());
                                }
                            }
                        }
                    }
                    _ => yield message?,
                }
            }
        }
    }
}
