use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use super::invisible::Invisible;

#[derive(Component)]
#[require(Sprite)]
#[derive(Default)]
pub struct Collisable;

#[derive(Event)]
pub struct CollidedEvent {
    pub entity1: Entity,
    pub entity2: Entity,
}

pub struct CollisablePlugin;

impl Plugin for CollisablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollidedEvent>()
            .add_systems(Update, check_collision);
    }
}

fn check_collision(
    mut event_writer: EventWriter<CollidedEvent>,
    collisable_query: Query<(Entity, &Transform, &Sprite), (With<Collisable>, Without<Invisible>)>,
) {
    for (i, (entity, transform, sprite)) in collisable_query.iter().enumerate() {
        let aabb = Aabb2d::new(
            transform.translation.truncate(),
            sprite.custom_size.unwrap() / 2.,
        );

        for (other_entity, other_transform, other_sprite) in collisable_query.iter().skip(i + 1) {
            let other_aabb = Aabb2d::new(
                other_transform.translation.truncate(),
                other_sprite.custom_size.unwrap() / 2.,
            );

            if aabb.intersects(&other_aabb) {
                event_writer.send(CollidedEvent {
                    entity1: entity,
                    entity2: other_entity,
                });
                return;
            }
        }
    }
}
