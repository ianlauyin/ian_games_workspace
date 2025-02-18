pub use bullet::*;
pub use collision::*;
pub use explosion::*;
pub use health::*;
pub use invisible::*;
pub use score::*;
pub use spaceship::*;
pub use ufo::*;

mod bullet;
mod collision;
mod explosion;
mod health;
mod invisible;
mod score;
mod spaceship;
mod ufo;

use bevy::prelude::{App, Plugin};
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {}
}
