use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::states::AppState;
use crate::ui::ZIndexMap;

#[derive(Event)]
pub struct AddScoreEvent;
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_score)
            .add_systems(OnExit(AppState::Game), cleanup_score)
            .add_observer(add_score);
    }
}

#[derive(Component)]
pub struct Score(pub u32);
fn setup_score(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.get_single().unwrap();
    commands
        .spawn((
            Anchor::BottomRight,
            Transform::from_xyz(
                window.width() / 2. - 20.,
                -window.height() / 2. + 20.,
                ZIndexMap::Text.value(),
            ),
            Text::new("Score:"),
        ))
        .with_child((Score(0), TextSpan::new("0")));
}

fn add_score(_: Trigger<AddScoreEvent>, mut score_query: Query<(&mut Score, &mut TextSpan)>) {
    let (mut score, mut text_span) = score_query.get_single_mut().unwrap();
    score.0 += 1;
    text_span.0 = score.0.to_string();
}

fn cleanup_score(mut commands: Commands, score_queries: Query<Entity, With<Score>>) {
    let entity = score_queries.get_single().unwrap();
    commands.entity(entity).despawn();
}
