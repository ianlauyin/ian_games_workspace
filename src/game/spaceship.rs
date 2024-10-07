use bevy::prelude::*;

use crate::AssetHandles;
use crate::constants::WINDOW_SIZE;
use crate::game::Velocity;
use crate::states::AppState;
use crate::ui::ZIndexMap;

#[derive(Component)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InPlay), setup_spaceship);
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
