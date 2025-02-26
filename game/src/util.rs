use bevy::prelude::*;
use std::f32::consts::PI;

pub fn angle_to_radian(angle: f32) -> f32 {
    angle * PI / 180.
}

pub fn cleanup_components<T: Component>(
    mut commands: Commands,
    component_q: Query<Entity, With<T>>,
) {
    for entity in component_q.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        };
    }
}
