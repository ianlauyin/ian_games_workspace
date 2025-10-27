use bevy::{ecs::component::Mutable, prelude::*};
use std::f32::consts::PI;

pub fn angle_to_radian(angle: f32) -> f32 {
    angle * PI / 180.
}

pub fn cleanup_components<T: Component>(
    mut commands: Commands,
    component_q: Query<Entity, With<T>>,
) {
    for entity in component_q.iter() {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn();
        };
    }
}

pub trait Position {
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, position: Vec2);
}

pub fn listen_position<T: Position + Component<Mutability = Mutable>>(
    mut query: Query<(&Transform, &mut T)>,
) {
    for (transform, mut position) in query.iter_mut() {
        position.set_position(transform.translation.xy());
    }
}
