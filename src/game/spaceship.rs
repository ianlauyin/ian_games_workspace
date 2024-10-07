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
        velocity.y = 0.
    }
    next_state.set(GameState::InPlay);
}
