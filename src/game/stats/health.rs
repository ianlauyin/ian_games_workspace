use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::states::AppState;

const INITIAL_HEALTH: u8 = 5;
#[derive(Component)]
pub struct Health(u8);

pub struct HealthPlugin;

#[derive(Event)]
pub struct HealthReduceEvent;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), reset_health)
            .add_event::<HealthReduceEvent>()
            .add_systems(Update, reduce_health.run_if(in_state(AppState::InPlay)));
    }
}

fn reset_health(mut commands: Commands, mut health_query: Query<&mut Health>) {
    if health_query.is_empty() {
        commands.spawn(Health(INITIAL_HEALTH));
        return;
    }
    let mut health = health_query.get_single_mut().unwrap();
    health.0 = INITIAL_HEALTH;
}

fn reduce_health(
    mut health_reduce_event: EventReader<HealthReduceEvent>,
    mut health_query: Query<&mut Health>,
) {
    for _ in health_reduce_event.read() {
        let health = health_query.get_single_mut().unwrap();
        health.0 -= 1;
    }
}
