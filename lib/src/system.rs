use bevy::prelude::*;

pub fn cleanup_components<T: Component>(
    mut commands: Commands,
    component_q: Query<Entity, With<T>>,
) {
    for entity in component_q.iter() {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.try_despawn();
        };
    }
}
