use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::states::AppState;
use crate::ui::{WINDOW_SIZE, ZIndexMap};

const INITIAL_HEALTH: u8 = 5;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_health)
            .add_systems(OnExit(AppState::Game), reset_health)
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

fn reset_health(mut health_query: Query<(&mut Health, &mut Text)>) {
    let (mut health, mut text) = health_query.get_single_mut().unwrap();
    health.0 = INITIAL_HEALTH;
    text.sections[1].value = INITIAL_HEALTH.to_string()
}

fn reduce_health(_: Trigger<HealthReduceEvent>, mut health_query: Query<(&mut Health, &mut Text)>) {
    let (mut health, mut text) = health_query.get_single_mut().unwrap();
    health.0 -= 1;
    text.sections[1].value = health.0.to_string();
}
