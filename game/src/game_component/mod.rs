mod health;
mod player;
mod score;
mod spaceship;

pub use health::Health;
pub use player::Player;
pub use score::Score;
pub use spaceship::Spaceship;

use bevy::prelude::{App, Plugin};
pub struct GameComponentPlugin;

impl Plugin for GameComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spaceship::SpaceshipPlugin);
    }
}
