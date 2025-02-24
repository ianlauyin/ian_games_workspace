use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

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
    mut commands: Commands,
    collisable_query: Query<(Entity, &Transform, &Sprite), With<Collisable>>,
) {
    for (i, (entity, transform, sprite)) in collisable_query.iter().enumerate() {
        let aabb = Aabb2d::new(
            transform.translation.truncate(),
            sprite.custom_size.unwrap(),
        );

        for (other_entity, other_transform, other_sprite) in collisable_query.iter().skip(i + 1) {
            let other_aabb = Aabb2d::new(
                other_transform.translation.truncate(),
                other_sprite.custom_size.unwrap(),
            );

            if aabb.intersects(&other_aabb) {
                commands.trigger(CollidedEvent {
                    entity1: entity,
                    entity2: other_entity,
                });
                return;
            }
        }
    }
}
