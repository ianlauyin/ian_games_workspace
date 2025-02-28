use bevy::prelude::*;
use shooting_game_shared::util::EdgeUtil;

use crate::components::{SelfPlayer, Spaceship, Velocity};

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

pub struct SpaceshipMovementPlugin;

impl Plugin for SpaceshipMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_spaceship_movement);
    }
}

pub fn handle_spaceship_movement(
    trigger: Trigger<SpaceShipMovementEvent>,
    mut spaceship_query: Query<(&mut Velocity, &Transform), (With<Spaceship>, With<SelfPlayer>)>,
) {
    let Ok((mut velocity, transform)) = spaceship_query.get_single_mut() else {
        return;
    };
    let Vec3 { x, y, z: _ } = transform.translation;
    let movement = &trigger.event().0;

    if *movement == SpaceShipMovement::Rest {
        velocity.x = 0.;
        velocity.y = 0.;
        return;
    }
    let edge = EdgeUtil::spaceship();

    velocity.x = match movement {
        SpaceShipMovement::Left if !edge.over_left_in(x) => -10.,
        SpaceShipMovement::UpLeft | SpaceShipMovement::DownLeft if !edge.over_left_in(x) => -7.,
        SpaceShipMovement::Right if !edge.over_right_in(x) => 10.,
        SpaceShipMovement::UpRight | SpaceShipMovement::DownRight if !edge.over_right_in(x) => 7.,
        _ => 0.,
    };

    velocity.y = match movement {
        SpaceShipMovement::Up if !edge.over_top_in(y) => 10.,
        SpaceShipMovement::UpLeft | SpaceShipMovement::UpRight if !edge.over_top_in(y) => 7.,
        SpaceShipMovement::Down if !edge.over_bottom_in(y) => -10.,
        SpaceShipMovement::DownLeft | SpaceShipMovement::DownRight if !edge.over_bottom_in(y) => {
            -7.
        }
        _ => 0.,
    };
}
