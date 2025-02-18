use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::game::{ExplosionEvent, InvisibleEvent, Spaceship};
use crate::states::{AppState, GameState};
use crate::ui::ZIndexMap;

const INITIAL_HEALTH: u8 = 3;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_health)
            .add_systems(OnExit(AppState::Game), cleanup_health)
            .add_observer(reduce_health);
    }
}

#[derive(Event)]
pub struct HealthReduceEvent;

#[derive(Component)]
struct Health(u8);

fn setup_health(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.get_single().unwrap();
    commands
        .spawn((
            Anchor::BottomLeft,
            Transform::from_xyz(
                -window.width() / 2. + 20.,
                -window.height() / 2. + 20.,
                ZIndexMap::Text.value(),
            ),
            Text::new("Health:"),
        ))
        .with_child((
            Health(INITIAL_HEALTH),
            TextSpan::new(INITIAL_HEALTH.to_string()),
        ));
}

fn reduce_health(
    _: Trigger<HealthReduceEvent>,
    mut health_query: Query<(&mut Health, &mut TextSpan)>,
    mut commands: Commands,
    spaceship_queries: Query<(Entity, &Transform), With<Spaceship>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut health, mut text_span) = health_query.get_single_mut().unwrap();
    health.0 -= 1;
    text_span.0 = health.0.to_string();
    if health.0 == 0 {
        let (entity, transform) = spaceship_queries.get_single().unwrap();
        commands.entity(entity).despawn();
        commands.trigger(ExplosionEvent {
            x: transform.translation.x,
            y: transform.translation.y,
        });
        return next_state.set(GameState::Result);
    }
    commands.trigger(InvisibleEvent);
}
fn cleanup_health(mut commands: Commands, health_queries: Query<Entity, With<Health>>) {
    let entity = health_queries.get_single().unwrap();
    commands.entity(entity).despawn();
}
