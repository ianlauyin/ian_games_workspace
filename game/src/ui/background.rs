use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::asset_loader::ImageHandles;
use crate::states::AppState;
use crate::ui::{ZIndexMap, MOBILE_WINDOW_SIZE};
use crate::util::Velocity;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), setup_background)
            .add_systems(
                Update,
                (check_stars_number, blinking_stars, cleanup_stars)
                    .run_if(in_state(AppState::Game).or_else(in_state(AppState::MainMenu))),
            );
    }
}

#[derive(Component)]
struct Stars {
    appearing: bool,
}

fn setup_background(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.05, 0., 0.05),
            custom_size: Some(MOBILE_WINDOW_SIZE),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., ZIndexMap::Background.value()),
        ..default()
    });
}

fn blinking_stars(mut stars_query: Query<(&mut Stars, &mut Sprite)>) {
    for (mut stars, mut sprite) in stars_query.iter_mut() {
        let alpha = sprite.color.alpha();
        if alpha >= 0.1 {
            stars.appearing = false;
        } else if alpha <= 0.001 {
            stars.appearing = true;
        }
        let new_alpha = if stars.appearing {
            alpha + 0.001
        } else {
            alpha - 0.001
        };
        sprite.color.set_alpha(new_alpha)
    }
}

fn cleanup_stars(
    mut commands: Commands,
    stars_query: Query<(Entity, &Transform), With<Stars>>,
    windows: Query<&Window>,
) {
    let window = windows.get_single().unwrap();
    for (entity, transform) in stars_query.iter() {
        if transform.translation.y <= -window.height() {
            commands.entity(entity).despawn();
        }
    }
}

fn check_stars_number(
    mut commands: Commands,
    stars_query: Query<&Transform, With<Stars>>,
    image_handles: Res<ImageHandles>,
    windows: Query<&Window>,
) {
    let stars_handle = image_handles.stars.clone();
    let window = windows.get_single().unwrap();
    if stars_query.is_empty() {
        spawn_star(&mut commands, stars_handle, window);
        return;
    }
    let Ok(transform) = stars_query.get_single() else {
        return;
    };
    if transform.translation.y < window.resolution.height() / 2.
        && star_random_generator(transform.translation.y)
    {
        spawn_star(&mut commands, stars_handle, window);
    }
}

fn spawn_star(commands: &mut Commands, stars_handle: Handle<Image>, window: &Window) {
    commands.spawn((
        Stars { appearing: true },
        Velocity { x: 0., y: -2. },
        Sprite {
            image: stars_handle,
            color: Color::default().with_alpha(0.01),
            ..default()
        },
        Transform {
            scale: Vec3::new(1.5, 1.5, 0.),
            translation: Vec3::new(0., window.height(), ZIndexMap::Stars.value()),
            ..default()
        },
    ));
}

fn star_random_generator(base_number: f32) -> bool {
    if base_number < 1. {
        return true;
    }
    let base_integer = base_number as u32;
    let mut rng = thread_rng();
    let random_value: u32 = rng.gen_range(0..base_integer);
    random_value == 1
}
