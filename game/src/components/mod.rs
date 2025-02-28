mod bullet;
mod collisable;
mod explosion;
mod health;
mod invisible;
mod player;
mod score;
mod spaceship;
mod ufo;
mod velocity;

use bevy::prelude::{App, Plugin};
pub use bullet::{Bullet, BulletTag};
pub use collisable::CollidedEvent;
pub use explosion::Explosion;
pub use health::Health;
pub use invisible::Invisible;
pub use player::{Player, SelfPlayer};
pub use score::Score;
pub use spaceship::Spaceship;
pub use ufo::{EnemyTag, UFO};
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
            invisible::InvisiblePlugin,
            bullet::BulletPlugin,
            player::PlayerPlugin,
        ));
    }
}
