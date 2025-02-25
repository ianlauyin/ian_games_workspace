use rocket::{futures::StreamExt, State};
use rocket_ws::{Channel, WebSocket};

use crate::message::ClientMessageHandler;
use crate::state::SharedGameState;

#[rocket::get("/game")]
pub async fn ws_handler<'a>(ws: WebSocket, game_state: &'a State<SharedGameState>) -> Channel<'a> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let (sender, receiver) = stream.split();

            // Add Sender to ServerMessageHandler
            match game_state.try_lock() {
                Ok(lock_state) => {
                    lock_state.new_player(sender).await;
                }
                Err(_) => {
                    println!("new player join failed")
                }
            };

            // Add Receiver to ClientMessageHandler
            let message_handler = ClientMessageHandler::new(game_state.inner().clone());
            message_handler.handle_messages(receiver).await;

            Ok(())
        })
    })
}
