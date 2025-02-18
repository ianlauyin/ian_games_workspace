use bevy::prelude::*;
use rand::{rng, Rng};

use crate::asset_loader::ImageHandles;
use crate::game::Score;
use crate::states::{AppState, GameState};
use crate::ui::{get_left_edge, get_right_edge, ZIndexMap, UFO_SIZE};
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
            .add_observer(remove_ufo);
    }
}

fn check_spawn_ufo(
    mut commands: Commands,
    image_handles: ResMut<ImageHandles>,
    ufo_query: Query<Entity, With<UFO>>,
    score_query: Query<&Score>,
    windows: Query<&Window>,
) {
    let ufo_number = ufo_query.iter().len();
    let Score(score) = score_query.get_single().unwrap();
    let window = windows.get_single().unwrap();
    let stage = match score {
        0..10 => Stage::Warmup,
        10..50 => Stage::One,
        50..100 => Stage::Two,
        100..150 => Stage::Three,
        150..200 => Stage::Four,
        200..250 => Stage::Five,
        _ => Stage::Six,
    };
    if ufo_number == 0 || stage.random_generator(ufo_number as f64) {
        spawn_ufo(&mut commands, image_handles.ufo.clone(), stage, window);
    }
}

enum Stage {
    Warmup,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Stage {
    fn random_generator(&self, existing_ufo: f64) -> bool {
        let mut rng = rng();
        return match self {
            Stage::Warmup => rng.random_bool(0.01),
            Stage::One | Stage::Two => rng.random_bool(1. / (existing_ufo * 10.)),
            Stage::Three | Stage::Four => rng.random_bool(1. / (existing_ufo * 3.)),
            Stage::Five | Stage::Six => rng.random_bool(1. / (existing_ufo)),
        };
    }
}

fn spawn_ufo(
    commands: &mut Commands,
    ufo_image_handle: Handle<Image>,
    stage: Stage,
    window: &Window,
) {
    let mut rng = rng();
    let velocity = match stage {
        Stage::Warmup | Stage::One => Velocity { x: 0., y: -3. },
        Stage::Two | Stage::Three => Velocity {
            x: if rng.gen_bool(0.5) { 3. } else { -3. },
            y: -3.,
        },
        Stage::Four | Stage::Five => Velocity {
            x: rng.gen_range(-5.0..5.0),
            y: rng.gen_range(-5.0..-3.0),
        },
        Stage::Six => Velocity {
            x: rng.gen_range(-10.0..10.0),
            y: rng.gen_range(-10.0..-5.0),
        },
    };

    let x = rng.gen_range(ufo_edge(get_left_edge)..ufo_edge(get_right_edge));
    commands.spawn((
        UFO,
        velocity,
        Sprite {
            image: ufo_image_handle,
            custom_size: Some(UFO_SIZE),
            ..default()
        },
        Transform::from_xyz(x, window.height() / 2. + UFO_SIZE.y, ZIndexMap::UFO.value()),
    ));
}

fn handle_horizontal_ufo(mut ufo_queries: Query<(&mut Velocity, &Transform), With<UFO>>) {
    for (mut velocity, transform) in ufo_queries.iter_mut() {
        if transform.translation.x <= ufo_edge(get_left_edge)
            || transform.translation.x >= ufo_edge(get_right_edge)
        {
            velocity.x = -velocity.x
        }
    }
}

fn clear_ufo(
    mut commands: Commands,
    ufo_queries: Query<(Entity, &Transform), With<UFO>>,
    windows: Query<&Window>,
) {
    let window = windows.get_single().unwrap();
    for (entity, transform) in ufo_queries.iter() {
        if transform.translation.y < -window.height() / 2. - UFO_SIZE.y {
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

fn ufo_edge(edge_function: impl FnOnce(f32) -> f32) -> f32 {
    edge_function(UFO_SIZE.x)
}
