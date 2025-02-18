use bevy::prelude::*;
use rand::{rng, Rng};

use crate::constant::{ZIndexMap, MOBILE_WINDOW_SIZE};
use crate::res::ImageHandles;
use crate::states::AppState;
use crate::ui_component::Blink;
use crate::util::Velocity;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_stars_number, cleanup_stars)
                .run_if(in_state(AppState::Game).or(in_state(AppState::MainMenu))),
        );
    }
}

#[derive(Component)]
struct Stars;

fn check_stars_number(
    mut commands: Commands,
    stars_query: Query<&Transform, With<Stars>>,
    image_handles: Res<ImageHandles>,
) {
    let stars_handle = image_handles.stars.clone();
    if stars_query.is_empty() {
        spawn_star(&mut commands, stars_handle);
        return;
    }
    let Ok(transform) = stars_query.get_single() else {
        return;
    };
    if transform.translation.y < MOBILE_WINDOW_SIZE.y / 2.
        && star_random_generator(transform.translation.y)
    {
        spawn_star(&mut commands, stars_handle);
    }
}

fn cleanup_stars(mut commands: Commands, stars_query: Query<(Entity, &Transform), With<Stars>>) {
    for (entity, transform) in stars_query.iter() {
        if transform.translation.y <= -MOBILE_WINDOW_SIZE.y {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_star(commands: &mut Commands, stars_handle: Handle<Image>) {
    commands.spawn((
        Stars,
        Blink::new(0.001, 0.1, 0.001),
        Velocity { x: 0., y: -2. },
        Sprite {
            image: stars_handle,
            color: Color::default().with_alpha(0.01),
            ..default()
        },
        Transform {
            scale: Vec3::new(1.5, 1.5, 0.),
            translation: Vec3::new(0., MOBILE_WINDOW_SIZE.y, ZIndexMap::STARS),
            ..default()
        },
    ));
}

fn star_random_generator(base_number: f32) -> bool {
    if base_number < 1. {
        return true;
    }
    let base_integer = base_number as u32;
    let mut rng = rng();
    let random_value: u32 = rng.random_range(0..base_integer);
    random_value == 1
}
