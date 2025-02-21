use bevy::prelude::*;

const INITIAL_HEALTH: u8 = 3;

#[derive(Component)]
pub struct Health(pub u8);

impl Health {
    pub fn new() -> Self {
        Self(INITIAL_HEALTH)
    }
    
    pub fn reduce(&mut self) {
        self.0 -= 1;
    }
}

// #[derive(Event)]
// pub struct HealthReduceEvent;

// fn reduce_health(
//     _: Trigger<HealthReduceEvent>,
//     mut health_query: Query<(&mut Health, &mut TextSpan)>,
//     mut commands: Commands,
//     spaceship_queries: Query<(Entity, &Transform), With<Spaceship>>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     let (mut health, mut text_span) = health_query.get_single_mut().unwrap();
//     health.0 -= 1;
//     text_span.0 = health.0.to_string();
//     if health.0 == 0 {
//         let (entity, transform) = spaceship_queries.get_single().unwrap();
//         commands.entity(entity).despawn();
//         commands.trigger(ExplosionEvent {
//             x: transform.translation.x,
//             y: transform.translation.y,
//         });
//         return next_state.set(GameState::Result);
//     }
//     commands.trigger(InvisibleEvent);
// }
// fn cleanup_health(mut commands: Commands, health_queries: Query<Entity, With<Health>>) {
//     let entity = health_queries.get_single().unwrap();
//     commands.entity(entity).despawn();
// }
