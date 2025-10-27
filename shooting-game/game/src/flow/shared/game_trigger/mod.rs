mod spaceship_movement;
use bevy::prelude::*;

pub use spaceship_movement::{SpaceShipMovement, SpaceShipMovementEvent};

pub struct GameTriggerPlugin;

impl Plugin for GameTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spaceship_movement::SpaceshipMovementPlugin);
    }
}
