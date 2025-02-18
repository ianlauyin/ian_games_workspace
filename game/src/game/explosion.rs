use bevy::prelude::*;
use bevy::time::Timer;

use crate::asset_loader::ImageHandles;
use crate::ui::ZIndexMap;

#[derive(Event)]
pub struct ExplosionEvent {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_explosion)
            .add_observer(create_explosion);
    }
}

#[derive(Component)]
struct Explosion {
    timer: Timer,
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
            commands.entity(entity).despawn();
        }
    }
}

fn create_explosion(
    trigger: Trigger<ExplosionEvent>,
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
) {
    let ExplosionEvent { x, y } = trigger.event();
    commands.spawn((
        Explosion {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
        Sprite {
            image: image_handles.explosion.clone(),
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        Transform::from_xyz(x.clone(), y.clone(), ZIndexMap::Explosion.value()),
    ));
}
