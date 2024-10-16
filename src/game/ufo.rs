use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::game::Velocity;
use crate::ImageHandles;
use crate::states::GameState;
use crate::ui::{LEFT_EDGE, RIGHT_EDGE, UFO_SIZE, WINDOW_SIZE};
use crate::ui::ZIndexMap;

#[derive(Component)]
pub struct UFO;

#[derive(Event)]
pub struct RemoveUFOEvent {
    pub(crate) ufo: Entity,
}

pub struct UFOPlugin;

impl Plugin for UFOPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (check_spawn_ufo, check_despawn_ufo).run_if(in_state(GameState::InPlay)),
        );
    }
}

fn check_spawn_ufo(
    mut commands: Commands,
    image_handles: ResMut<ImageHandles>,
    ufo_query: Query<Entity, With<UFO>>,
) {
    let mut rng = thread_rng();
    if ufo_query.is_empty() || rng.gen_range(1..100) == 1 {
        let x = rng.gen_range(LEFT_EDGE..RIGHT_EDGE);
        commands.spawn((
            UFO,
            Velocity { x: 0., y: -3. },
            SpriteBundle {
                texture: image_handles.ufo.clone(),
                sprite: Sprite {
                    custom_size: Some(UFO_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, WINDOW_SIZE.y, ZIndexMap::UFO.value()),
                ..default()
            },
        ));
    }
}

fn check_despawn_ufo(mut commands: Commands, ufo_queries: Query<(Entity, &Transform), With<UFO>>) {
    for (entity, transform) in ufo_queries.iter() {
        if transform.translation.y < -WINDOW_SIZE.y / 1.1 {
            commands.entity(entity).despawn();
        }
    }
}
