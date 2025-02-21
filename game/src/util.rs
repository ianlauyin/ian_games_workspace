use bevy::prelude::*;
use std::f32::consts::PI;

pub fn angle_to_radian(angle: f32) -> f32 {
    angle * PI / 180.
}

pub fn cleanup<T: Component>(mut commands: Commands, component_q: Query<Entity, With<T>>) {
    for entity in component_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
