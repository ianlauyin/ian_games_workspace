use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::asset_loader::ImageHandles;
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
                check_spawn_ufo.run_if(in_state(GameState::InPlay)),
            )
            .add_systems(OnExit(AppState::Game), cleanup_ufo)
            .observe(remove_ufo);
    }
}

fn check_spawn_ufo(
    mut commands: Commands,
    image_handles: ResMut<ImageHandles>,
    ufo_query: Query<Entity, With<UFO>>,
) {
    let mut rng = thread_rng();
    let ufo_number = ufo_query.iter().len();
    if ufo_number == 0 || rng.gen_range(1..ufo_number * 5) == 1 {
        spawn_ufo(&mut commands, image_handles.ufo.clone());
    }
}

fn spawn_ufo(commands: &mut Commands, ufo_image_handle: Handle<Image>) {
    let mut rng = thread_rng();
    let x = rng.gen_range(LEFT_EDGE..RIGHT_EDGE);
    commands.spawn((
        UFO,
        Velocity { x: 0., y: -3. },
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
