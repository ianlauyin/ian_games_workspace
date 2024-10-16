use bevy::app::App;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use crate::game::Bullet;
use crate::game::score::AddScoreEvent;
use crate::game::ufo::UFO;
use crate::ui::{BULLET_SIZE, UFO_SIZE};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_bullet_ufo);
    }
}

#[derive(Component)]
pub struct CollisionObject;

fn check_bullet_ufo(
    mut commands: Commands,
    bullet_queries: Query<(Entity, &Transform), With<Bullet>>,
    ufo_queries: Query<(Entity, &Transform), With<UFO>>,
) {
    for (bullet_entity, bullet_transform) in bullet_queries.iter() {
        let bullet_aabb = Aabb2d::new(
            bullet_transform.translation.truncate(),
            Vec2::new(BULLET_SIZE.x / 2., BULLET_SIZE.y / 2.),
        );
        for (ufo_entity, ufo_transform) in ufo_queries.iter() {
            let ufo_aabb = Aabb2d::new(
                ufo_transform.translation.truncate(),
                Vec2::new(UFO_SIZE.x / 2., BULLET_SIZE.y / 2.),
            );
            if !bullet_aabb.intersects(&ufo_aabb) {
                continue;
            } else {
                commands.trigger(AddScoreEvent);
                commands.entity(bullet_entity).despawn();
                commands.entity(ufo_entity).despawn();
                return;
            }
        }
    }
}
