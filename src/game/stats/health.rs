use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::states::{AppState, GameState};

const INITIAL_HEALTH: u8 = 5;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), reset_health)
            .add_event::<HealthReduceEvent>()
            .add_systems(Update, reduce_health.run_if(in_state(GameState::InPlay)));
    }
}

#[derive(Event)]
pub struct HealthReduceEvent;

#[derive(Component)]
struct Health(u8);

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
        let mut health = health_query.get_single_mut().unwrap();
        health.0 -= 1;
    }
}
