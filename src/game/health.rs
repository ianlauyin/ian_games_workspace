use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::game::{ExplosionEvent, Spaceship};
use crate::states::{AppState, GameState};
use crate::ui::{WINDOW_SIZE, ZIndexMap};

const INITIAL_HEALTH: u8 = 3;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_health)
            .add_systems(OnExit(AppState::Game), cleanup_health)
            .observe(reduce_health);
    }
}

#[derive(Event)]
pub struct HealthReduceEvent;

#[derive(Component)]
struct Health(u8);

fn setup_health(mut commands: Commands) {
    commands.spawn((
        Health(INITIAL_HEALTH),
        Text2dBundle {
            text_anchor: Anchor::BottomLeft,
            transform: Transform::from_xyz(
                -WINDOW_SIZE.x / 2. + 20.,
                -WINDOW_SIZE.y / 2. + 20.,
                ZIndexMap::Text.value(),
            ),
            text: Text::from_sections([
                TextSection::new("Health:", TextStyle::default()),
                TextSection::new(INITIAL_HEALTH.to_string(), TextStyle::default()),
            ]),
            ..default()
        },
    ));
}

fn reduce_health(
    _: Trigger<HealthReduceEvent>,
    mut health_query: Query<(&mut Health, &mut Text)>,
    mut commands: Commands,
    spaceship_queries: Query<(Entity, &Transform), With<Spaceship>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut health, mut text) = health_query.get_single_mut().unwrap();
    health.0 -= 1;
    text.sections[1].value = health.0.to_string();
    if health.0 == 0 {
        let (entity, transform) = spaceship_queries.get_single().unwrap();
        commands.entity(entity).despawn();
        commands.trigger(ExplosionEvent {
            x: transform.translation.x,
            y: transform.translation.y,
        });
        next_state.set(GameState::Result);
    }
}
fn cleanup_health(mut commands: Commands, health_queries: Query<Entity, With<Health>>) {
    let entity = health_queries.get_single().unwrap();
    commands.entity(entity).despawn();
}
