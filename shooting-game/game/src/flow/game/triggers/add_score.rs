use bevy::prelude::*;

use crate::components::{Player, Score};

#[derive(Event)]
pub struct AddScoreEvent {
    player: u8,
    amount: u8,
}

impl AddScoreEvent {
    pub fn new(player: u8, amount: u8) -> Self {
        Self { amount, player }
    }
}

pub struct AddScorePlugin;

impl Plugin for AddScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_score);
    }
}

fn add_score(ev: On<AddScoreEvent>, mut score_query: Query<(&mut Score, &Player)>) {
    for (mut score, player) in score_query.iter_mut() {
        if player.0 == ev.player {
            score.add(ev.amount);
        }
    }
}
