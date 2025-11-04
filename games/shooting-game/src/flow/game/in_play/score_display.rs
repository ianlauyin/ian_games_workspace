use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::components::Score;
use crate::states::GameState;
use game_lib::system::cleanup_components;

pub struct ScoreDisplayPlugin;

impl Plugin for ScoreDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InPlay), display_score)
            .add_systems(
                Update,
                update_score_text.run_if(in_state(GameState::InPlay)),
            )
            .add_systems(
                OnExit(GameState::InPlay),
                cleanup_components::<ScoreDisplay>,
            );
    }
}

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct PlayerScoreText;

fn display_score(mut commands: Commands, score_q: Query<&Score>) {
    commands
        .spawn((
            ScoreDisplay,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(5.),
                top: Val::Px(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|score_display| {
            let Ok(score) = score_q.single() else {
                warn!("Score not found in display_score");
                return;
            };
            score_display
                .spawn(Text::new("Score:"))
                .with_child((PlayerScoreText, TextSpan::new(score.0.to_string())));
        });
}

fn update_score_text(
    score_q: Query<&Score, Changed<Score>>,
    mut player_score_text_q: Query<&mut TextSpan, With<PlayerScoreText>>,
) {
    if score_q.is_empty() {
        return;
    }
    let Ok(score) = score_q.single() else {
        warn!("Score not found in update_score_text");
        return;
    };
    let Ok(mut text_span) = player_score_text_q.single_mut() else {
        warn!("Player score text not found in update_score_text");
        return;
    };
    text_span.0 = score.0.to_string();
}
