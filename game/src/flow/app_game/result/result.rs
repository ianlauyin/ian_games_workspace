use bevy::app::App;
use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

use crate::game::Score;
use crate::states::{AppState, GameState};
use crate::ui_component::ZIndexMap;

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
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 0.5)),
            Transform {
                translation: Vec3::default().with_z(ZIndexMap::MainMenu.value()),
                ..default()
            },
        ))
        .with_children(|background| {
            background.spawn((
                Text::new(format!("Final Score: {score}")),
                Transform::from_translation(Vec3::ZERO.with_z(ZIndexMap::Text.value())),
            ));

            background
                .spawn(Node {
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    row_gap: Val::Px(5.),
                    ..default()
                })
                .with_children(|return_container| {
                    return_container.spawn((
                        EndTips { appearing: false },
                        Text::new("Click Return to return to main menu"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform {
                            translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                            ..default()
                        },
                    ));
                    return_container
                        .spawn((
                            ReturnButton,
                            Interaction::default(),
                            Node {
                                align_self: AlignSelf::FlexEnd,
                                width: Val::Px(120.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(2.)),
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 1.)),
                            BorderColor::from(Color::BLACK),
                        ))
                        .with_children(|button_node| {
                            button_node.spawn(Text::new("Return"));
                        });
                });
        });
}

fn handle_return_button_interaction(
    mut commands: Commands,
    mut return_button_query: Query<(&Interaction, &mut BackgroundColor), With<ReturnButton>>,
    mut window_query: Query<Entity, With<Window>>,
    result_query: Query<Entity, With<Result>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let window_entity = window_query.get_single_mut().unwrap();
    let (interaction, mut background) = return_button_query.get_single_mut().unwrap();
    match interaction {
        Interaction::None => {
            commands.entity(window_entity).insert(CursorIcon::default());
            background.0.set_alpha(1.)
        }
        Interaction::Hovered => {
            commands
                .entity(window_entity)
                .insert(CursorIcon::System(SystemCursorIcon::Pointer));
            background.0.set_alpha(0.5)
        }
        Interaction::Pressed => {
            commands.entity(window_entity).insert(CursorIcon::default());
            let result = result_query.get_single().unwrap();
            commands.entity(result).despawn_recursive();
            next_state.set(AppState::MainMenu)
        }
    }
}

fn end_tips_animation(mut end_tip_queries: Query<(&mut TextColor, &mut EndTips)>) {
    let (mut text_color, mut start_tips) = end_tip_queries.get_single_mut().unwrap();
    let original_text_alpha = text_color.alpha();
    let new_text_alpha = if start_tips.appearing {
        original_text_alpha + 0.02
    } else {
        original_text_alpha - 0.02
    };
    text_color.set_alpha(new_text_alpha);
    if new_text_alpha < 0. {
        start_tips.appearing = true
    }
    if new_text_alpha > 1. {
        start_tips.appearing = false
    }
}
