mod health;
mod player;
mod score;
mod spaceship;
mod ufo;

use bevy::prelude::{App, Plugin};
pub use health::Health;
pub use player::Player;
pub use score::Score;
pub use spaceship::Spaceship;
pub use ufo::UFO;
pub struct GameComponentPlugin;

impl Plugin for GameComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((spaceship::SpaceshipPlugin, ufo::UFOPlugin));
    }
}
