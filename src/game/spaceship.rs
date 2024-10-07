use bevy::prelude::*;

use crate::AssetHandles;
use crate::constants::WINDOW_SIZE;
use crate::game::Velocity;
use crate::states::{AppState, GameState};
use crate::ui::ZIndexMap;

#[derive(Component)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InPlay), setup_spaceship)
            .add_systems(
                Update,
                check_spaceship_position
                    .run_if(in_state(GameState::Ready))
                    .run_if(in_state(AppState::InPlay)),
            )
            .add_systems(
                Update,
                check_spaceship_interaction
                    .run_if(in_state(GameState::InPlay))
                    .run_if(in_state(AppState::InPlay)),
            );
    }
}

fn setup_spaceship(mut commands: Commands, asset_handles: Res<AssetHandles>) {
    commands.spawn((
        Spaceship,
        Velocity { x: 0., y: 5. },
        SpriteBundle {
            texture: asset_handles.spaceship.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -WINDOW_SIZE.y / 1.5, ZIndexMap::SpaceShip.value()),
            ..default()
        },
    ));
}

fn check_spaceship_position(
    mut next_state: ResMut<NextState<GameState>>,
    mut spaceship_query: Query<(&Transform, &mut Velocity), With<Spaceship>>,
) {
    let (transform, mut velocity) = spaceship_query.get_single_mut().unwrap();
    if transform.translation.y >= -WINDOW_SIZE.y / 2.5 {
        velocity.y = 0.;
        next_state.set(GameState::InPlay);
    }
}

fn check_spaceship_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    mut spaceship_query: Query<(&mut Velocity, &mut Transform), With<Spaceship>>,
) {
    let (velocity, transform) = spaceship_query.get_single_mut().unwrap();
    let move_right = if keys.pressed(KeyCode::ArrowRight) {
        Some(true)
    } else if keys.pressed(KeyCode::ArrowLeft) {
        Some(false)
    } else {
        None
    };
    handle_spaceship_movement(velocity, transform, move_right);
}

const SPACESHIP_ROTATE_AXIS: Vec3 = Vec3::new(1., 1., -0.5);

fn handle_spaceship_movement(
    mut velocity: Mut<Velocity>,
    mut transform: Mut<Transform>,
    move_right: Option<bool>,
) {
    let (new_velocity, new_rotation): (f32, Quat) = {
        match move_right {
            None => (0., default()),
            Some(move_right) => {
                if move_right {
                    (5., Quat::from_axis_angle(SPACESHIP_ROTATE_AXIS, 0.4))
                } else {
                    (-5., Quat::from_axis_angle(SPACESHIP_ROTATE_AXIS, -0.4))
                }
            }
        }
    };
    velocity.x = new_velocity;
    transform.rotation = new_rotation
}
