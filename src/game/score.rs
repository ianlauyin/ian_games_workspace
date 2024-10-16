use bevy::prelude::*;

use crate::states::AppState;

#[derive(Event)]
pub struct AddScoreEvent;
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .observe(add_score)
            .add_systems(OnEnter(AppState::MainMenu), reset_score);
    }
}

#[derive(Resource)]
struct Score(u32);

fn add_score(_: Trigger<AddScoreEvent>, mut score: ResMut<Score>) {
    score.0 += 1;
}

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}
