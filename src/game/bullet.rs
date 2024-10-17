use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::asset_loader::MeshHandles;
use crate::ui::{BULLET_SIZE, WINDOW_SIZE};
use crate::ui::ZIndexMap;
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
            .observe(shoot_bullet)
            .observe(remove_bullet);
    }
}

#[derive(Bundle)]
struct BulletBundle {
    bullet: Bullet,
    velocity: Velocity,
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl BulletBundle {
    fn new(
        x: f32,
        y: f32,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            bullet: Bullet,
            velocity: Velocity { x: 0., y: 10. },
            mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform {
                    translation: Vec3::new(x, y, ZIndexMap::Bullet.value()),
                    scale: BULLET_SIZE.extend(1.),
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn shoot_bullet(
    trigger: Trigger<ShootBulletEvent>,
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
) {
    let ShootBulletEvent { x, y } = trigger.event();
    let (mesh, material) = mesh_handles.bullet.clone();
    commands.spawn(BulletBundle::new(
        x.clone() - 20.,
        y.clone(),
        mesh.clone().into(),
        material.clone(),
    ));
    commands.spawn(BulletBundle::new(
        x.clone() + 20.,
        y.clone(),
        mesh.into(),
        material,
    ));
}

fn clear_bullet(mut commands: Commands, bullet_queries: Query<(Entity, &Transform), With<Bullet>>) {
    for (entity, transform) in bullet_queries.iter() {
        if transform.translation.y > WINDOW_SIZE.y / 2. + 200. {
            commands.entity(entity).despawn();
        }
    }
}

fn remove_bullet(trigger: Trigger<RemoveBulletEvent>, mut commands: Commands) {
    let RemoveBulletEvent { bullet } = trigger.event();
    commands.entity(*bullet).despawn()
}
