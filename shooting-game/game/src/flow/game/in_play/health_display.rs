use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::components::Health;
use crate::states::GameState;
use crate::util::cleanup_components;

pub struct HealthDisplayPlugin;

impl Plugin for HealthDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InPlay), display_health)
            .add_systems(
                Update,
                update_health_text.run_if(in_state(GameState::InPlay)),
            )
            .add_systems(
                OnExit(GameState::InPlay),
                cleanup_components::<HealthDisplay>,
            );
    }
}

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct PlayerHealthText;

fn display_health(mut commands: Commands, health_q: Query<&Health>) {
    commands
        .spawn((
            HealthDisplay,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                left: Val::Px(5.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|health_display| {
            let Ok(health) = health_q.single() else {
                warn!("Health not found in display_health");
                return;
            };
            health_display
                .spawn(Text::new("Health:"))
                .with_child((PlayerHealthText, TextSpan::new(health.0.to_string())));
        });
}

fn update_health_text(
    health_q: Query<&Health, Changed<Health>>,
    mut player_health_text_q: Query<&mut TextSpan, With<PlayerHealthText>>,
) {
    if health_q.is_empty() {
        return;
    }
    let Ok(health) = health_q.single() else {
        warn!("Health not found in update_health_text");
        return;
    };
    let Ok(mut text_span) = player_health_text_q.single_mut() else {
        warn!("Player health text not found in update_health_text");
        return;
    };
    text_span.0 = health.0.to_string();
}
