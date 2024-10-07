use bevy::prelude::*;

use crate::AssetHandles;
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
    commands.spawn((
        Background,
        SpriteBundle {
            texture: asset_handles.stars.clone(),
            ..default()
        },
    ));
}
