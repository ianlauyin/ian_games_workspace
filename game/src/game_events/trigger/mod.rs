mod spaceship_movement;
use bevy::prelude::{App, Plugin};

pub use spaceship_movement::{SpaceShipMovement, SpaceShipMovementEvent};
pub struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spaceship_movement::SpaceshipMovementPlugin);
    }
}
