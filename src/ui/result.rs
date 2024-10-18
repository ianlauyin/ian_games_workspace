use bevy::app::App;
use bevy::prelude::*;

use crate::game::Score;
use crate::states::{AppState, GameState};
use crate::ui::ZIndexMap;

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Result), show_result)
            .add_systems(
                Update,
                (end_tips_animation, handle_return).run_if(in_state(GameState::Result)),
            );
    }
}

#[derive(Component)]
struct Result;

#[derive(Component)]
struct EndTips {
    appearing: bool,
}
fn show_result(mut commands: Commands, score_query: Query<&Score>) {
    let Score(score) = score_query.get_single().unwrap();
    commands
        .spawn((
            Result,
            NodeBundle {
                style: Style {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(80.),
                    height: Val::Percent(80.),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 0.5)),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::MainMenu.value()),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(format!("Final Score: {score}"), TextStyle::default()),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
                ..default()
            });
            parent.spawn((
                EndTips { appearing: false },
                TextBundle {
                    style: Style {
                        margin: UiRect::top(Val::Percent(50.)),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "Press Enter to return to main menu",
                        TextStyle::default(),
                    ),
                    transform: Transform {
                        translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn handle_return(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    result_query: Query<Entity, With<Result>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        let result = result_query.get_single().unwrap();
        commands.entity(result).despawn_recursive();
        next_state.set(AppState::MainMenu)
    }
}

fn end_tips_animation(mut end_tip_queries: Query<(&mut Text, &mut EndTips)>) {
    let (mut text, mut start_tips) = end_tip_queries.get_single_mut().unwrap();
    let original_text_alpha = text.sections[0].style.color.alpha();
    let new_text_alpha = if start_tips.appearing {
        original_text_alpha + 0.02
    } else {
        original_text_alpha - 0.02
    };
    text.sections[0].style.color.set_alpha(new_text_alpha);
    if new_text_alpha < 0. {
        start_tips.appearing = true
    }
    if new_text_alpha > 1. {
        start_tips.appearing = false
    }
}
