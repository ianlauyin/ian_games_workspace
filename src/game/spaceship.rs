use bevy::prelude::*;

use crate::asset_loader::ImageHandles;
use crate::game::ShootBulletEvent;
use crate::states::{AppState, GameState};
use crate::ui::{LEFT_EDGE, RIGHT_EDGE, SPACESHIP_SIZE, WINDOW_SIZE, ZIndexMap};
use crate::util::Velocity;

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
                FixedUpdate,
                (handle_spaceship_interaction, handle_shoot_bullet)
                    .run_if(in_state(GameState::InPlay)),
            );
    }
}

fn setup_spaceship(mut commands: Commands, image_handles: Res<ImageHandles>) {
    commands.spawn((
        Spaceship,
        Velocity { x: 0., y: 5. },
        SpriteBundle {
            texture: image_handles.spaceship.clone(),
            sprite: Sprite {
                custom_size: Some(SPACESHIP_SIZE),
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

    velocity.x = match (
        keys.pressed(KeyCode::ArrowLeft),
        keys.pressed(KeyCode::ArrowRight),
    ) {
        (false, true) if transform.translation.x <= RIGHT_EDGE => 10.,
        (true, false) if transform.translation.x >= LEFT_EDGE => -10.,
        _ => 0.,
    };
}

fn handle_shoot_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    spaceship_query: Query<&Transform, With<Spaceship>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let transform = spaceship_query.get_single().unwrap();
        let Vec3 { x, y, .. } = transform.translation;
        commands.trigger(ShootBulletEvent { x, y })
    }
}
