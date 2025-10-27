use bevy::prelude::*;

use crate::{
    components::{Health, Player, Score, SelfPlayer},
    states::OnlineGameState,
    util::cleanup_components,
};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OnlineGameState::InPlay), setup_display)
            .add_systems(
                Update,
                (update_health_text, update_score_text).run_if(in_state(OnlineGameState::InPlay)),
            )
            .add_systems(
                OnExit(OnlineGameState::InPlay),
                cleanup_components::<InfoDisplay>,
            );
    }
}

#[derive(Component)]
struct InfoDisplay;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct ScoreText;

fn setup_display(
    mut commands: Commands,
    health_without_self_q: Query<(&Health, &Player), Without<SelfPlayer>>,
    score_without_self_q: Query<(&Score, &Player), Without<SelfPlayer>>,
    self_health_q: Query<(&Health, &Player), With<SelfPlayer>>,
    self_score_q: Query<(&Score, &Player), With<SelfPlayer>>,
) {
    commands
        .spawn((
            InfoDisplay,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                left: Val::Px(5.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|your_info| {
            your_info.spawn(Text::new("Your"));
            if let Ok((health, player)) = self_health_q.single() {
                your_info.spawn(Text::new("Health: ")).with_child((
                    player.clone(),
                    HealthText,
                    TextSpan::new(health.0.to_string()),
                ));
            }
            if let Ok((score, player)) = self_score_q.single() {
                your_info.spawn(Text::new("Score: ")).with_child((
                    player.clone(),
                    ScoreText,
                    TextSpan::new(score.0.to_string()),
                ));
            }
        });
    commands
        .spawn((
            InfoDisplay,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.),
                left: Val::Px(5.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|your_info| {
            your_info.spawn(Text::new("Opponent"));
            if let Ok((health, player)) = health_without_self_q.single() {
                your_info.spawn(Text::new("Health: ")).with_child((
                    player.clone(),
                    HealthText,
                    TextSpan::new(health.0.to_string()),
                ));
            }
            if let Ok((score, player)) = score_without_self_q.single() {
                your_info.spawn(Text::new("Score: ")).with_child((
                    player.clone(),
                    ScoreText,
                    TextSpan::new(score.0.to_string()),
                ));
            }
        });
}

fn update_health_text(
    health_q: Query<(&Health, &Player), Changed<Health>>,
    mut health_text_q: Query<(&mut TextSpan, &Player), With<HealthText>>,
) {
    if health_q.is_empty() {
        return;
    }
    for (health, target_player) in health_q.iter() {
        for (mut text_span, player) in health_text_q.iter_mut() {
            if target_player.0 == player.0 {
                text_span.0 = health.0.to_string();
            }
        }
    }
}

fn update_score_text(
    score_q: Query<(&Score, &Player), Changed<Score>>,
    mut score_text_q: Query<(&mut TextSpan, &Player), With<ScoreText>>,
) {
    if score_q.is_empty() {
        return;
    }
    for (score, target_player) in score_q.iter() {
        for (mut text_span, player) in score_text_q.iter_mut() {
            if target_player.0 == player.0 {
                text_span.0 = score.0.to_string();
            }
        }
    }
}
