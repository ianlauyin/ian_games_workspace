use std::sync::Arc;

use rocket::{
    futures::{SinkExt, StreamExt},
    tokio::sync::Mutex,
    State,
};
use rocket_ws::{Channel, Message, WebSocket};

use super::{
    message::{Receiver, Sender},
    state::ArcGameState,
};

#[rocket::get("/game")]
pub async fn ws_handler<'a>(ws: WebSocket, game_state: &'a State<ArcGameState>) -> Channel<'a> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let (sender, receiver) = stream.split();
            let player_sender = Arc::new(Mutex::new(sender));

            match game_state.try_lock() {
                Ok(lock_state) => {
                    lock_state.new_player(player_sender.clone()).await;
                }
                Err(_) => {
                    panic!("new player join failed")
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
