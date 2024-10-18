use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::asset_loader::ImageHandles;
use crate::game::Score;
use crate::states::{AppState, GameState};
use crate::ui::{LEFT_EDGE, RIGHT_EDGE, UFO_SIZE, WINDOW_SIZE, ZIndexMap};
use crate::util::Velocity;

#[derive(Component)]
pub struct UFO;

#[derive(Event)]
pub struct RemoveUFOEvent {
    pub(crate) ufo: Entity,
}

pub struct UFOPlugin;

impl Plugin for UFOPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, clear_ufo)
            .add_systems(
                FixedUpdate,
                (check_spawn_ufo, handle_horizontal_ufo).run_if(in_state(GameState::InPlay)),
            )
            .add_systems(OnExit(AppState::Game), cleanup_ufo)
            .observe(remove_ufo);
    }
}

fn check_spawn_ufo(
    mut commands: Commands,
    image_handles: ResMut<ImageHandles>,
    ufo_query: Query<Entity, With<UFO>>,
    score_query: Query<&Score>,
) {
    let ufo_number = ufo_query.iter().len();
    let Score(score) = score_query.get_single().unwrap();
    let stage = match score {
        0..10 => Stage::Warmup,
        10..50 => Stage::One,
        50..100 => Stage::Two,
        100..150 => Stage::Three,
        _ => Stage::Four,
    };
    if ufo_number == 0 || stage.random_generator(ufo_number as f64) {
        spawn_ufo(&mut commands, image_handles.ufo.clone(), stage);
    }
}

enum Stage {
    Warmup,
    One,
    Two,
    Three,
    Four,
}

impl Stage {
    fn random_generator(&self, existing_ufo: f64) -> bool {
        let mut rng = thread_rng();
        return match self {
            Stage::Warmup => rng.gen_bool(0.01),
            Stage::One | Stage::Two => rng.gen_bool(1. / (existing_ufo * 10.)),
            Stage::Three | Stage::Four => rng.gen_bool(1. / (existing_ufo * 5.)),
        };
    }
}

fn spawn_ufo(commands: &mut Commands, ufo_image_handle: Handle<Image>, stage: Stage) {
    let mut rng = thread_rng();
    let velocity = match stage {
        Stage::Warmup | Stage::One => Velocity { x: 0., y: -3. },
        Stage::Two | Stage::Three => Velocity {
            x: if rng.gen_bool(0.5) { 3. } else { -3. },
            y: -3.,
        },
        Stage::Four => Velocity {
            x: rng.gen_range(-5.0..5.0),
            y: rng.gen_range(-5.0..-3.0),
        },
    };

    let x = rng.gen_range(LEFT_EDGE..RIGHT_EDGE);
    commands.spawn((
        UFO,
        velocity,
        SpriteBundle {
            texture: ufo_image_handle,
            sprite: Sprite {
                custom_size: Some(UFO_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(x, WINDOW_SIZE.y / 2. + 50., ZIndexMap::UFO.value()),
            ..default()
        },
    ));
}

fn handle_horizontal_ufo(mut ufo_queries: Query<(&mut Velocity, &Transform), With<UFO>>) {
    for (mut velocity, transform) in ufo_queries.iter_mut() {
        if transform.translation.x <= LEFT_EDGE || transform.translation.x >= RIGHT_EDGE {
            velocity.x = -velocity.x
        }
    }
}

fn clear_ufo(mut commands: Commands, ufo_queries: Query<(Entity, &Transform), With<UFO>>) {
    for (entity, transform) in ufo_queries.iter() {
        if transform.translation.y < -WINDOW_SIZE.y / 2. - 50. {
            commands.entity(entity).despawn();
        }
    }
}

fn remove_ufo(trigger: Trigger<RemoveUFOEvent>, mut commands: Commands) {
    let RemoveUFOEvent { ufo } = trigger.event();
    let Some(mut entity_commands) = commands.get_entity(*ufo) else {
        return;
    };
    entity_commands.despawn();
}

fn cleanup_ufo(mut commands: Commands, ufo_queries: Query<Entity, With<UFO>>) {
    for entity in ufo_queries.iter() {
        commands.entity(entity).despawn();
    }
}
