use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::states::AppState;
use crate::ui::{WINDOW_SIZE, ZIndexMap};

#[derive(Event)]
pub struct AddScoreEvent;
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_score)
            .add_systems(OnExit(AppState::Game), cleanup_score)
            .observe(add_score);
    }
}

#[derive(Component)]
struct Score(u32);
fn setup_score(mut commands: Commands) {
    commands.spawn((
        Score(0),
        Text2dBundle {
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(
                WINDOW_SIZE.x / 2. - 20.,
                -WINDOW_SIZE.y / 2. + 20.,
                ZIndexMap::Text.value(),
            ),
            text: Text::from_sections([
                TextSection::new("Score:", TextStyle::default()),
                TextSection::new("0", TextStyle::default()),
            ]),
            ..default()
        },
    ));
}

fn add_score(_: Trigger<AddScoreEvent>, mut score_query: Query<(&mut Score, &mut Text)>) {
    let (mut score, mut text) = score_query.get_single_mut().unwrap();
    score.0 += 1;
    text.sections[1].value = score.0.to_string();
}

fn cleanup_score(mut commands: Commands, score_queries: Query<Entity, With<Score>>) {
    let entity = score_queries.get_single().unwrap();
    commands.entity(entity).despawn();
}
