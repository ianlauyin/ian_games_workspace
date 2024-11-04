use bevy::app::App;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use crate::game::{
    AddScoreEvent, Bullet, ExplosionEvent, HealthReduceEvent, Invisible, RemoveBulletEvent,
    RemoveUFOEvent, Spaceship, UFO,
};
use crate::states::GameState;
use crate::ui::{get_bullet_size, get_spaceship_size, get_ufo_size};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_bullet_ufo, check_spaceship_ufo).run_if(in_state(GameState::InPlay)),
        );
    }
}

fn check_bullet_ufo(
    mut commands: Commands,
    bullet_queries: Query<(Entity, &Transform), With<Bullet>>,
    ufo_queries: Query<(Entity, &Transform), With<UFO>>,
    windows: Query<&Window>,
) {
    let window = windows.get_single().unwrap();
    for (bullet_entity, bullet_transform) in bullet_queries.iter() {
        let bullet_aabb = Aabb2d::new(
            bullet_transform.translation.truncate(),
            get_bullet_size(window.width()) / 2.,
        );
        for (ufo_entity, ufo_transform) in ufo_queries.iter() {
            let ufo_aabb = Aabb2d::new(
                ufo_transform.translation.truncate(),
                get_ufo_size(window.width()) / 2.,
            );
            if !bullet_aabb.intersects(&ufo_aabb) {
                continue;
            }
            commands.trigger(AddScoreEvent);
            commands.trigger(RemoveBulletEvent {
                bullet: bullet_entity,
            });
            commands.trigger(RemoveUFOEvent { ufo: ufo_entity });
            commands.trigger(ExplosionEvent {
                x: ufo_transform.translation.x,
                y: ufo_transform.translation.y,
            });
            return;
        }
    }
}

fn check_spaceship_ufo(
    mut commands: Commands,
    spaceship_queries: Query<&Transform, (With<Spaceship>, Without<Invisible>)>,
    ufo_queries: Query<(Entity, &Transform), With<UFO>>,
    windows: Query<&Window>,
) {
    if spaceship_queries.is_empty() {
        return;
    }
    let window = windows.get_single().unwrap();
    let spaceship_transform = spaceship_queries.get_single().unwrap();
    let spaceship_aabb = Aabb2d::new(
        spaceship_transform.translation.truncate(),
        get_spaceship_size(window.width()) / 2.,
    );
    for (ufo_entity, ufo_transform) in ufo_queries.iter() {
        let ufo_aabb = Aabb2d::new(
            ufo_transform.translation.truncate(),
            get_ufo_size(window.width()) / 2.,
        );
        if !spaceship_aabb.intersects(&ufo_aabb) {
            continue;
        }
        commands.trigger(HealthReduceEvent);
        commands.trigger(RemoveUFOEvent { ufo: ufo_entity });
        return;
    }
}
