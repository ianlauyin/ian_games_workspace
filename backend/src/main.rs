use rocket::tokio::sync::Mutex;
use std::sync::Arc;

mod game;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .manage(Arc::new(Mutex::new(game::GameState::new())))
        .mount("/ws", rocket::routes![game::ws_handler])
        .launch()
        .await?;

    Ok(())
}
