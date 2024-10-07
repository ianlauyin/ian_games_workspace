use bevy::prelude::*;

use crate::AssetHandles;
use crate::constants::WINDOW_SIZE;
use crate::states::AppState;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::LoadAsset), setup_background);
    }
}

#[derive(Component)]
struct Background;

fn setup_background(mut commands: Commands, asset_handles: Res<AssetHandles>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.1, 0., 0.1),
            custom_size: Some(WINDOW_SIZE.truncate()),
            ..default()
        },
        ..default()
    });
    commands.spawn((
        Background,
        SpriteBundle {
            texture: asset_handles.stars.clone(),
            transform: Transform::from_scale(Vec3::splat(1.5)),
            ..default()
        },
    ));
}
