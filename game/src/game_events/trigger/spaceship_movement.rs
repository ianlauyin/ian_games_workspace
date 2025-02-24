use bevy::prelude::*;

use crate::{
    constant::SPACESHIP_SIZE, game_component::Spaceship, ui_component::Velocity, util::EdgeUtil,
};

pub struct SpaceshipMovementPlugin;

impl Plugin for SpaceshipMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_spaceship_movement);
    }
}

#[derive(Event)]
pub struct SpaceShipMovementEvent(pub SpaceShipMovement);

#[derive(Eq, PartialEq)]
pub enum SpaceShipMovement {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Rest,
}

pub fn handle_spaceship_movement(
    trigger: Trigger<SpaceShipMovementEvent>,
    mut spaceship_query: Query<(&mut Velocity, &Transform), With<Spaceship>>,
) {
    let Ok((mut velocity, transform)) = spaceship_query.get_single_mut() else {
        return;
    };

    let Vec3 { x, y, z: _ } = transform.translation;

    if trigger.event().0 == SpaceShipMovement::Rest {
        velocity.x = 0.;
        velocity.y = 0.;
        return;
    }
    let edge = EdgeUtil::new(SPACESHIP_SIZE);

    velocity.x = match trigger.event().0 {
        SpaceShipMovement::Left if !edge.over_left_in(x) => -10.,
        SpaceShipMovement::UpLeft | SpaceShipMovement::DownLeft if !edge.over_left_in(x) => -7.,
        SpaceShipMovement::Right if !edge.over_right_in(x) => 10.,
        SpaceShipMovement::UpRight | SpaceShipMovement::DownRight if !edge.over_right_in(x) => 7.,
        _ => 0.,
    };

    velocity.y = match trigger.event().0 {
        SpaceShipMovement::Up if !edge.over_top_in(y) => 10.,
        SpaceShipMovement::UpLeft | SpaceShipMovement::UpRight if !edge.over_top_in(y) => 7.,
        SpaceShipMovement::Down if !edge.over_bottom_in(y) => -10.,
        SpaceShipMovement::DownLeft | SpaceShipMovement::DownRight if !edge.over_bottom_in(y) => {
            -7.
        }
        _ => 0.,
    };
}
