use rocket::{futures::StreamExt, State};
use rocket_ws::{Channel, WebSocket};

use crate::{message::Receiver, state::SharedGameState};

#[rocket::get("/game")]
pub async fn ws_handler<'a>(ws: WebSocket, game_state: &'a State<SharedGameState>) -> Channel<'a> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let (sender, receiver) = stream.split();

            match game_state.try_lock() {
                Ok(lock_state) => {
                    lock_state.new_player(sender).await;
                }
                Err(_) => {
                    println!("new player join failed")
                }
            };

            handle_client_message(receiver).await;

            Ok(())
        })
    })
}

async fn handle_client_message(mut receiver: Receiver) {
    while let Some(message) = receiver.next().await {
        if let Ok(msg) = message {
            println!("Received message: {:?}", msg);
        }
    }
}
