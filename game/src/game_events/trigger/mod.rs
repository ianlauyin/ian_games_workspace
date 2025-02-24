mod add_score;
mod health_reduce;
mod remove_ufo;
mod spaceship_movement;
use bevy::prelude::{App, Plugin};

pub use add_score::AddScoreEvent;
pub use health_reduce::HealthReduceEvent;
pub use remove_ufo::RemoveUFOEvent;
pub use spaceship_movement::{SpaceShipMovement, SpaceShipMovementEvent};
pub struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spaceship_movement::SpaceshipMovementPlugin,
            health_reduce::HealthReducePlugin,
            add_score::AddScorePlugin,
            remove_ufo::RemoveUFOPlugin,
        ));
    }
}
