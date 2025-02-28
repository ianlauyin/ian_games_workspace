use bevy::prelude::*;
use shooting_game_shared::util::EdgeUtil;

use crate::{
    components::{BulletTag, UFO},
    constant::BULLET_SIZE,
};

pub struct OutScreenCleanupPlugin;

impl Plugin for OutScreenCleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cleanup_on_out_screen);
    }
}

fn cleanup_on_out_screen(
    mut commands: Commands,
    ufo_query: Query<(Entity, &Transform), With<UFO>>,
    bullet_query: Query<(Entity, &Transform), With<BulletTag>>,
) {
    let ufo_edge = EdgeUtil::ufo();
    for (entity, transform) in ufo_query.iter() {
        if ufo_edge.over_bottom_out(transform.translation.y) {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
    }

    let bullet_edge = EdgeUtil::new(BULLET_SIZE);
    for (entity, transform) in bullet_query.iter() {
        if bullet_edge.over_top_out(transform.translation.y) {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
    }
}
