use bevy::prelude::*;

#[derive(Component)]
pub struct Score(pub u8);

impl Score {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add(&mut self, amount: u8) {
        self.0 += amount;
    }
}

// #[derive(Event)]
// pub struct AddScoreEvent;

// fn setup_score(mut commands: Commands, windows: Query<&Window>) {
//     let window = windows.get_single().unwrap();
//     commands
//         .spawn((
//             Anchor::BottomRight,
//             Transform::from_xyz(
//                 window.width() / 2. - 20.,
//                 -window.height() / 2. + 20.,
//                 ZIndexMap::Text.value(),
//             ),
//             Text::new("Score:"),
//         ))
//         .with_child((Score(0), TextSpan::new("0")));
// }

// fn add_score(_: Trigger<AddScoreEvent>, mut score_query: Query<(&mut Score, &mut TextSpan)>) {
//     let (mut score, mut text_span) = score_query.get_single_mut().unwrap();
//     score.0 += 1;
//     text_span.0 = score.0.to_string();
// }

// fn cleanup_score(mut commands: Commands, score_queries: Query<Entity, With<Score>>) {
//     let entity = score_queries.get_single().unwrap();
//     commands.entity(entity).despawn();
// }
