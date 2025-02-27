mod add_score;
mod health_reduce;
mod spaceship_movement;
use bevy::prelude::*;

pub use add_score::AddScoreEvent;
pub use health_reduce::HealthReduceEvent;
pub use spaceship_movement::{SpaceShipMovement, SpaceShipMovementEvent};

pub struct GameTriggerPlugin;

impl Plugin for GameTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            add_score::AddScorePlugin,
            health_reduce::HealthReducePlugin,
            spaceship_movement::SpaceshipMovementPlugin,
        ));
    }
}
