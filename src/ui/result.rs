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
                (end_tips_animation, handle_return_button_interaction)
                    .run_if(in_state(GameState::Result)),
            );
    }
}

#[derive(Component)]
struct Result;

#[derive(Component)]
struct EndTips {
    appearing: bool,
}

#[derive(Component)]
struct ReturnButton;

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
                        "Click Return to return to main menu",
                        TextStyle::default(),
                    ),
                    transform: Transform {
                        translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                        ..default()
                    },
                    ..default()
                },
            ));
            parent
                .spawn((
                    ReturnButton,
                    Interaction::default(),
                    NodeBundle {
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            width: Val::Px(100.),
                            height: Val::Px(50.),
                            border: UiRect::all(Val::Px(2.)),
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 1.)),
                        border_color: BorderColor::from(Color::BLACK),
                        ..default()
                    },
                ))
                .with_children(|button_node| {
                    button_node.spawn(TextBundle {
                        text: Text::from_section("Return", TextStyle::default()),
                        ..default()
                    });
                });
        });
}

fn handle_return_button_interaction(
    mut commands: Commands,
    mut return_button_query: Query<(&Interaction, &mut BackgroundColor), With<ReturnButton>>,
    mut window_query: Query<&mut Window>,
    result_query: Query<Entity, With<Result>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut window = window_query.get_single_mut().unwrap();
    let (interaction, mut background) = return_button_query.get_single_mut().unwrap();
    match interaction {
        Interaction::None => {
            window.cursor.icon = CursorIcon::default();
            background.0.set_alpha(1.)
        }
        Interaction::Hovered => {
            window.cursor.icon = CursorIcon::Pointer;
            background.0.set_alpha(0.5)
        }
        Interaction::Pressed => {
            window.cursor.icon = CursorIcon::default();
            let result = result_query.get_single().unwrap();
            commands.entity(result).despawn_recursive();
            next_state.set(AppState::MainMenu)
        }
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
