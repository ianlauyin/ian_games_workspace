use bevy::prelude::*;

use crate::AssetHandles;
use crate::constants::WINDOW_SIZE;
use crate::states::AppState;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::LoadAsset), setup_background)
            .add_systems(
                FixedUpdate,
                blinking_star.run_if(in_state(AppState::InPlay)),
            );
    }
}

#[derive(Component)]
struct Background {
    appearing: bool,
}

fn setup_background(mut commands: Commands, asset_handles: Res<AssetHandles>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.05, 0., 0.05),
            custom_size: Some(WINDOW_SIZE.truncate()),
            ..default()
        },
        ..default()
    });
    commands.spawn((
        Background { appearing: true },
        SpriteBundle {
            texture: asset_handles.stars.clone(),
            sprite: Sprite {
                color: Color::default().with_alpha(0.01),
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(1.5)),
            ..default()
        },
    ));
}

fn blinking_star(mut background_query: Query<(&mut Background, &mut Sprite)>) {
    let (mut background, mut sprite) = background_query.get_single_mut().unwrap();
    let alpha = sprite.color.alpha();
    println!("{alpha}");
    if alpha >= 0.5 {
        background.appearing = false;
    } else if alpha <= 0.1 {
        background.appearing = true;
    }
    let new_alpha = if background.appearing {
        alpha + 0.005
    } else {
        alpha - 0.001
    };
    sprite.color.set_alpha(new_alpha)
}
