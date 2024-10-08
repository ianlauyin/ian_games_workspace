use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::states::{AppState, GameState};

const INITIAL_HEALTH: u8 = 3;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Health(INITIAL_HEALTH))
            .add_event::<HealthReduceEvent>()
            .add_systems(Update, reduce_health.run_if(in_state(GameState::InPlay)))
            .add_systems(OnExit(AppState::InPlay), reset_health);
    }
}

#[derive(Event)]
pub struct HealthReduceEvent;

#[derive(Resource)]
struct Health(u8);

fn reset_health(mut health: ResMut<Health>) {
    health.0 = INITIAL_HEALTH
}

fn reduce_health(
    mut health_reduce_event: EventReader<HealthReduceEvent>,
    mut health: ResMut<Health>,
) {
    for _ in health_reduce_event.read() {
        health.0 -= 1;
    }
}
