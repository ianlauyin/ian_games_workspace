use bevy::prelude::*;

use crate::constants::WINDOW_SIZE;
use crate::game::Velocity;
use crate::ImageHandles;
use crate::states::{AppState, GameState};
use crate::ui::ZIndexMap;

#[derive(Component)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_spaceship)
            .add_systems(
                Update,
                check_spaceship_position.run_if(in_state(GameState::Ready)),
            )
            .add_systems(
                Update,
                handle_spaceship_interaction.run_if(in_state(GameState::InPlay)),
            );
    }
}

fn setup_spaceship(mut commands: Commands, asset_handles: Res<ImageHandles>) {
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

fn handle_spaceship_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    mut spaceship_query: Query<(&mut Velocity, &Transform), With<Spaceship>>,
) {
    let (mut velocity, transform) = spaceship_query.get_single_mut().unwrap();
    let limit_edge = WINDOW_SIZE.x / 2. - 50.;

    velocity.x = match (
        keys.pressed(KeyCode::ArrowLeft),
        keys.pressed(KeyCode::ArrowRight),
    ) {
        (false, true) if transform.translation.x <= limit_edge => 5.,
        (true, false) if transform.translation.x >= -limit_edge => -5.,
        _ => 0.,
    };
}
