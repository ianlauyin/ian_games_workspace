use bevy::prelude::*;
use bevy::time::Timer;

use crate::constant::{ZIndex::EXPLOSION, EXPLOSION_SIZE};
use crate::res::ImageHandles;

#[derive(Component)]
#[require(Transform)]
pub struct Explosion {
    position: Vec2,
    timer: Timer,
}

impl Explosion {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_explosion)
            .add_observer(handle_explosion_on_added);
    }
}

fn handle_explosion_on_added(
    ev: Trigger<OnAdd, Explosion>,
    mut commands: Commands,
    explosion_query: Query<&Explosion>,
    image_handles: Res<ImageHandles>,
) {
    let explosion = explosion_query.get(ev.target()).unwrap();
    if let Ok(mut entity_commands) = commands.get_entity(ev.target()) {
        entity_commands.insert((
            Sprite {
                image: image_handles.explosion.clone(),
                custom_size: Some(EXPLOSION_SIZE),
                ..default()
            },
            Transform::from_translation(explosion.position.extend(EXPLOSION.z_value())),
        ));
    }
}

fn apply_explosion(
    mut commands: Commands,
    mut explosion_queries: Query<(Entity, &mut Explosion, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut explosion, mut transform) in explosion_queries.iter_mut() {
        explosion.timer.tick(time.delta());
        transform.scale.x += 0.01;
        transform.scale.y += 0.01;
        if explosion.timer.finished() {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
    }
}
