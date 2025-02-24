use bevy::prelude::*;

use crate::{
    components::{Explosion, Health, Spaceship},
    states::GameState,
};

pub struct FinishPlugin;

impl Plugin for FinishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_finish.run_if(in_state(GameState::InPlay)));
    }
}

fn check_finish(
    mut commands: Commands,
    health_q: Query<&Health>,
    mut next_state: ResMut<NextState<GameState>>,
    spaceship_q: Query<(Entity, &Spaceship)>,
) {
    let Ok(health) = health_q.get_single() else {
        panic!("Health not found");
    };
    if let Ok((entity, spaceship)) = spaceship_q.get_single() {
        if health.0 == 0 {
            commands.spawn(Explosion::new(spaceship.get_position()));
            commands.entity(entity).despawn_recursive();
            next_state.set(GameState::Result);
        }
    }
}
