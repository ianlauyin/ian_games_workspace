use bevy::prelude::*;

use crate::components::{Health, Player};

#[derive(Event)]
pub struct HealthReduceEvent {
    player: u8,
}

impl HealthReduceEvent {
    pub fn new(player: u8) -> Self {
        Self { player }
    }
}

pub struct HealthReducePlugin;

impl Plugin for HealthReducePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(reduce_health);
    }
}

fn reduce_health(ev: On<HealthReduceEvent>, mut health_query: Query<(&mut Health, &Player)>) {
    for (mut health, player) in health_query.iter_mut() {
        if player.0 == ev.player {
            if health.0 > 0 {
                health.reduce();
            }
        }
    }
}
