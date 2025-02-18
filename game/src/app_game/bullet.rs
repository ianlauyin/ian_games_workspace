use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;

use crate::states::AppState;
use crate::ui_component::{ZIndexMap, BULLET_SIZE};
use crate::util::Velocity;

#[derive(Component)]
pub struct Bullet;

#[derive(Event)]
pub struct ShootBulletEvent {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Event)]
pub struct RemoveBulletEvent {
    pub(crate) bullet: Entity,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, clear_bullet)
            .add_systems(OnExit(AppState::Game), cleanup_bullet)
            .add_observer(shoot_bullet)
            .add_observer(remove_bullet);
    }
}

#[derive(Bundle)]
struct BulletBundle {
    bullet: Bullet,
    velocity: Velocity,
    transform: Transform,
    sprite: Sprite,
}

impl BulletBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            bullet: Bullet,
            velocity: Velocity { x: 0., y: 10. },
            transform: Transform {
                translation: Vec3::new(x, y, ZIndexMap::Bullet.value()),
                scale: BULLET_SIZE.extend(1.),
                ..default()
            },
            sprite: Sprite::from_color(Color::from(YELLOW), BULLET_SIZE),
        }
    }
}

fn shoot_bullet(trigger: Trigger<ShootBulletEvent>, mut commands: Commands) {
    let ShootBulletEvent { x, y } = trigger.event();
    commands.spawn(BulletBundle::new(x.clone() - 20., y.clone()));
    commands.spawn(BulletBundle::new(x.clone() + 20., y.clone()));
}

fn clear_bullet(
    mut commands: Commands,
    bullet_queries: Query<(Entity, &Transform), With<Bullet>>,
    windows: Query<&Window>,
) {
    let window = windows.get_single().unwrap();
    for (entity, transform) in bullet_queries.iter() {
        if transform.translation.y > window.height() / 2. + BULLET_SIZE.y {
            commands.entity(entity).despawn();
        }
    }
}

fn remove_bullet(trigger: Trigger<RemoveBulletEvent>, mut commands: Commands) {
    let RemoveBulletEvent { bullet } = trigger.event();
    let Some(mut entity_commands) = commands.get_entity(*bullet) else {
        return;
    };
    entity_commands.despawn();
}

fn cleanup_bullet(mut commands: Commands, ufo_queries: Query<Entity, With<Bullet>>) {
    for entity in ufo_queries.iter() {
        commands.entity(entity).despawn();
    }
}
