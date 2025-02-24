use bevy::prelude::*;

use crate::{components::Health, states::GameState};

pub struct FinishPlugin;

impl Plugin for FinishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_finish);
    }
}

fn check_finish(health_q: Query<&Health>, mut next_state: ResMut<NextState<GameState>>) {
    for health in health_q.iter() {
        if health.0 == 0 {
            next_state.set(GameState::Result);
        }
    }
}
