pub use spaceship::SpaceshipPlugin;
pub use stats::health::{HealthPlugin, HealthReduceEvent};
pub use stats::velocity::{Velocity, VelocityPlugin};

mod spaceship;
mod stats;
