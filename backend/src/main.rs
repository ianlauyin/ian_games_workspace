use rocket::tokio::time::sleep;
use rocket::tokio::{spawn, sync::RwLock};
use state::{Cycle, SharedGameState};
use std::sync::Arc;
use std::time::Duration;

mod handler;
mod message;
mod state;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let game_state = Arc::new(RwLock::new(state::GameState::default()));

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
        let mut locked_state = game_state.write().await;
        let cycle = locked_state.check_cycle().await;
        drop(locked_state);
        let sleep_millis = match cycle {
            Cycle::Playing => 20,
            _ => 500,
        };
        sleep(Duration::from_millis(sleep_millis)).await
    }
}
