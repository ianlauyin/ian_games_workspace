use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::components::{Health, Player};
use crate::states::GameState;
use crate::util::cleanup_components;

pub struct HealthDisplayPlugin;

impl Plugin for HealthDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InPlay), display_health)
            .add_systems(
                OnExit(GameState::InPlay),
                cleanup_components::<HealthDisplay>,
            )
            .add_observer(update_health_text);
    }
}

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct PlayerHealthText(u8);

fn display_health(mut commands: Commands, health_q: Query<(&Health, &Player)>) {
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
            health_display.spawn(Text::new("Health:"));
            for (health, player) in health_q.iter() {
                health_display
                    .spawn((Text::new(format!("Player {}: ", player.0)),))
                    .with_child((
                        PlayerHealthText(player.0),
                        TextSpan::new(health.0.to_string()),
                    ));
            }
        });
}

fn update_health_text(
    ev: Trigger<OnReplace, Health>,
    health_q: Query<(&Health, &Player)>,
    mut player_health_text_q: Query<(&mut TextSpan, &PlayerHealthText)>,
) {
    let Ok((health, player)) = health_q.get(ev.entity()) else {
        warn!("Cannot find player with updated health");
        return;
    };

    for (mut text_span, player_health_text) in player_health_text_q.iter_mut() {
        if player_health_text.0 == player.0 {
            text_span.0 = health.0.to_string();
        }
    }
}
