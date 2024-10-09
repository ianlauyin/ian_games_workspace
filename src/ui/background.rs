use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::constants::WINDOW_SIZE;
use crate::ImageHandles;
use crate::states::AppState;
use crate::ui::ZIndexMap;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), setup_background)
            .add_systems(
                FixedUpdate,
                (check_stars, animate_stars).run_if(in_state(AppState::Game)),
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
            custom_size: Some(WINDOW_SIZE),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., ZIndexMap::Background.value()),
        ..default()
    });
}

fn animate_stars(
    mut commands: Commands,
    mut stars_query: Query<(Entity, &mut Stars, &mut Sprite, &mut Transform)>,
) {
    for (entity, stars, sprite, transform) in stars_query.iter_mut() {
        if transform.translation.y <= -WINDOW_SIZE.y {
            despawn_star(&mut commands, entity);
            continue;
        }
        moving_stars(transform);
        blinking_stars(stars, sprite);
    }
}

fn moving_stars(mut transform: Mut<Transform>) {
    let translation = transform.translation;
    let new_translation = translation.with_y(translation.y - 3.);
    transform.translation = new_translation;
}

fn blinking_stars(mut stars: Mut<Stars>, mut sprite: Mut<Sprite>) {
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

fn check_stars(
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
    if star_random_generator(transform.translation.y) {
        spawn_star(&mut commands, stars_handle);
    }
}
fn spawn_star(commands: &mut Commands, stars_handle: Handle<Image>) {
    commands.spawn((
        Stars { appearing: true },
        SpriteBundle {
            texture: stars_handle,
            sprite: Sprite {
                color: Color::default().with_alpha(0.01),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(1.5, 1.5, 0.),
                translation: Vec3::new(0., WINDOW_SIZE.y - 100., ZIndexMap::Stars.value()),
                ..default()
            },
            ..default()
        },
    ));
}

fn despawn_star(commands: &mut Commands, star: Entity) {
    commands.entity(star).despawn();
}

fn star_random_generator(base_number: f32) -> bool {
    if base_number < 0. {
        return true;
    }
    let base_integer = base_number as u32;
    let mut rng = thread_rng();
    let random_value: u32 = rng.gen_range(0..base_integer);
    random_value == 1
}
