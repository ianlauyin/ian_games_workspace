mod collisable;
mod explosion;
mod health;
mod player;
mod score;
mod spaceship;
mod ufo;
mod velocity;

use bevy::prelude::{App, Plugin};
pub use collisable::{CollidedEvent, Collisable};
pub use explosion::Explosion;
pub use health::Health;
pub use player::Player;
pub use score::Score;
pub use spaceship::Spaceship;
pub use ufo::UFO;
pub use velocity::Velocity;
pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spaceship::SpaceshipPlugin,
            ufo::UFOPlugin,
            collisable::CollisablePlugin,
            explosion::ExplosionPlugin,
            velocity::VelocityPlugin,
        ));
    }
}
