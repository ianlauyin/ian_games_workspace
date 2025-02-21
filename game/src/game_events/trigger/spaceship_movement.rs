use bevy::prelude::*;

use crate::{game_component::Spaceship, ui_component::Velocity};

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

    velocity.x = match trigger.event().0 {
        SpaceShipMovement::Left if !meet_left_edge(x) => -10.,
        SpaceShipMovement::UpLeft | SpaceShipMovement::DownLeft if !meet_left_edge(x) => -7.,
        SpaceShipMovement::Right if !meet_right_edge(x) => 10.,
        SpaceShipMovement::UpRight | SpaceShipMovement::DownRight if !meet_right_edge(x) => 7.,
        _ => 0.,
    };

    velocity.y = match trigger.event().0 {
        SpaceShipMovement::Up if !meet_top_edge(y) => 10.,
        SpaceShipMovement::UpLeft | SpaceShipMovement::UpRight if !meet_top_edge(y) => 7.,
        SpaceShipMovement::Down if !meet_bottom_edge(y) => -10.,
        SpaceShipMovement::DownLeft | SpaceShipMovement::DownRight if !meet_bottom_edge(y) => -7.,
        _ => 0.,
    };
}

fn meet_top_edge(position: f32) -> bool {
    false
}

fn meet_bottom_edge(position: f32) -> bool {
    false
}

fn meet_left_edge(position: f32) -> bool {
    false
}

fn meet_right_edge(position: f32) -> bool {
    false
}
