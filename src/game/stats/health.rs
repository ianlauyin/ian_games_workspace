use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::states::AppState;

const INITIAL_HEALTH: u8 = 3;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Health(INITIAL_HEALTH))
            .add_systems(OnExit(AppState::Game), reset_health)
            .observe(reduce_health);
    }
}

#[derive(Event)]
pub struct HealthReduceEvent;

#[derive(Resource)]
struct Health(u8);

fn reset_health(mut health: ResMut<Health>) {
    health.0 = INITIAL_HEALTH
}

fn reduce_health(_: Trigger<HealthReduceEvent>, mut health: ResMut<Health>) {
    health.0 -= 1;
}
