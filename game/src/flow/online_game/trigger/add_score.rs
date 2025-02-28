use bevy::prelude::*;

use crate::components::{Player, Score};

#[derive(Event)]
pub struct AddScoreEvent {
    pub player_tag: u8,
    pub score: u8,
}

pub struct AddScorePlugin;

impl Plugin for AddScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_score);
    }
}

fn add_score(ev: Trigger<AddScoreEvent>, mut score_q: Query<(&mut Score, &Player)>) {
    let event = ev.event();
    for (mut score, player) in score_q.iter_mut() {
        if player.0 == event.player_tag {
            score.0 = event.score;
            break;
        }
    }
}
