use bevy::app::App;
use bevy::prelude::*;

use crate::asset_loader::MeshHandles;
use crate::game::Bullet;
use crate::game::ufo::UFO;

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
    mesh_handles: Res<MeshHandles>,
) {
    for (bullet_entity, bullet_transform) in bullet_queries.iter() {
        for (ufo_entity, ufo_transform) in ufo_queries.iter() {}
    }
}
