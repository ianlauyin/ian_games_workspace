pub use bullet::{Bullet, BulletPlugin, ShootBulletEvent};
pub use collision::CollisionPlugin;
pub use explosion::{ExplosionEvent, ExplosionPlugin};
pub use score::ScorePlugin;
pub use spaceship::SpaceshipPlugin;
pub use stats::health::HealthPlugin;
pub use stats::velocity::{Velocity, VelocityPlugin};
pub use ufo::UFOPlugin;

mod bullet;
mod collision;
mod explosion;
mod score;
mod spaceship;
mod stats;
mod ufo;
