use rocket::tokio::sync::Mutex;
use std::sync::Arc;

mod handler;
mod message;
mod state;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .manage(Arc::new(Mutex::new(state::GameState::default())))
        .mount("/ws", rocket::routes![handler::ws_handler])
        .launch()
        .await?;

    Ok(())
}
