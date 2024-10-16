pub use bullet::{BulletPlugin, ShootBulletEvent};
pub use spaceship::SpaceshipPlugin;
pub use stats::health::HealthPlugin;
pub use stats::velocity::{Velocity, VelocityPlugin};
pub use ufo::UFOPlugin;

mod bullet;
mod spaceship;
mod stats;
mod ufo;
