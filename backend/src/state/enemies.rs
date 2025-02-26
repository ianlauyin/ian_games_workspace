use rocket::tokio::sync::Mutex;

#[derive(Default)]
pub struct Enemies(Mutex<Vec<Enemy>>);

struct Enemy {
    pub tag: u8,
    pub position: (f32, f32),
}
