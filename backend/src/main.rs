mod game;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/ws", rocket::routes![game::ws_handler])
        .launch()
        .await?;

    Ok(())
}
