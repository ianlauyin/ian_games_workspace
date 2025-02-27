use rocket::tokio::time::sleep;
use rocket::tokio::{spawn, sync::Mutex};
use state::SharedGameState;
use std::sync::Arc;
use std::time::Duration;

mod handler;
mod message;
mod state;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let game_state = Arc::new(Mutex::new(state::GameState::default()));

    let game_state_clone = Arc::clone(&game_state);
    spawn(game_loop(game_state_clone));

    rocket::build()
        .manage(game_state)
        .mount("/ws", rocket::routes![handler::ws_handler])
        .launch()
        .await?;

    Ok(())
}

async fn game_loop(game_state: SharedGameState) {
    loop {
        let mut locked_state = game_state.lock().await;
        locked_state.check_cycle().await;
        drop(locked_state);
        sleep(Duration::from_millis(500)).await;
    }
}
