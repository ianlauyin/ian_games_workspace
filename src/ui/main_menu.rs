use bevy::app::App;
use bevy::prelude::*;

use crate::control::{ControlMode, ControlOption};
use crate::states::AppState;
use crate::ui::ZIndexMap;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), show_main_menu)
            .add_systems(
                Update,
                (
                    start_tips_animation,
                    handle_check_box_interaction,
                    handle_start_button_interaction,
                )
                    .run_if(in_state(AppState::MainMenu)),
            )
            .observe(change_control);
    }
}

#[derive(Event)]
struct ChangeControlEvent {
    is_click_mode: bool,
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct StartTips {
    appearing: bool,
}
#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct CheckBox {
    is_click_mode: bool,
}

fn show_main_menu(mut commands: Commands, control_option: Res<ControlOption>) {
    let is_click_mode = control_option.mode == ControlMode::Click;
    commands
        .spawn((
            MainMenu,
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
        .with_children(|menu_background| {
            menu_background.spawn(TextBundle {
                text: Text::from_section("Press Arrow move", TextStyle::default()),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
                ..default()
            });
            menu_background.spawn(TextBundle {
                text: Text::from_section("Press Space to shoot bullet", TextStyle::default()),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
                ..default()
            });
            menu_background.spawn(TextBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(50.)),
                    ..default()
                },
                text: Text::from_section(
                    "Whenever the ufo crash you, you will lose health.",
                    TextStyle::default(),
                ),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
                ..default()
            });
            menu_background.spawn(TextBundle {
                text: Text::from_section(
                    "Every 50 score will increase the difficulty",
                    TextStyle::default(),
                ),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
                ..default()
            });
            menu_background
                .spawn((NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        row_gap: Val::Px(10.),
                        flex_grow: 1.,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|option_node| {
                    option_node.spawn((
                        CheckBox {
                            is_click_mode: true,
                        },
                        Interaction::default(),
                        TextBundle {
                            style: Style {
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            },
                            text: Text::from_sections([
                                TextSection::new(
                                    if is_click_mode { "-> " } else { "" },
                                    TextStyle::default(),
                                ),
                                TextSection::new("Use Clicking to play", TextStyle::default()),
                            ]),
                            ..default()
                        },
                    ));
                    option_node.spawn((
                        CheckBox {
                            is_click_mode: false,
                        },
                        Interaction::default(),
                        TextBundle {
                            style: Style {
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            },
                            text: Text::from_sections([
                                TextSection::new(
                                    if is_click_mode { "" } else { "-> " },
                                    TextStyle::default(),
                                ),
                                TextSection::new("Use Keyboard to play", TextStyle::default()),
                            ]),
                            ..default()
                        },
                    ));
                    option_node.spawn((
                        StartTips { appearing: false },
                        TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            text: Text::from_section(
                                "Click Start to start the game",
                                TextStyle::default(),
                            ),
                            transform: Transform {
                                translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                    option_node
                        .spawn((
                            StartButton,
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
                                background_color: BackgroundColor::from(Color::srgba(
                                    0.1, 0.1, 0.1, 1.,
                                )),
                                border_color: BorderColor::from(Color::BLACK),
                                ..default()
                            },
                        ))
                        .with_children(|button_node| {
                            button_node.spawn(TextBundle {
                                text: Text::from_section("Start", TextStyle::default()),
                                ..default()
                            });
                        });
                });
        });
}

fn handle_check_box_interaction(
    mut commands: Commands,
    mut check_box_query: Query<(&Interaction, &mut Text, &CheckBox)>,
) {
    for (interaction, mut text, check_box) in check_box_query.iter_mut() {
        match interaction {
            Interaction::Hovered => text
                .sections
                .iter_mut()
                .for_each(|text_section| text_section.style.color.set_alpha(0.5)),
            Interaction::None => text
                .sections
                .iter_mut()
                .for_each(|text_section| text_section.style.color.set_alpha(1.0)),
            Interaction::Pressed => commands.trigger(ChangeControlEvent {
                is_click_mode: check_box.is_click_mode,
            }),
        }
    }
}

fn change_control(
    trigger: Trigger<ChangeControlEvent>,
    mut check_box_query: Query<(&mut Text, &CheckBox)>,
    mut control_option: ResMut<ControlOption>,
) {
    let ChangeControlEvent { is_click_mode } = trigger.event();
    let current_is_click_mode = control_option.mode == ControlMode::Click;
    if *is_click_mode == current_is_click_mode {
        return;
    }
    control_option.mode = if *is_click_mode {
        ControlMode::Click
    } else {
        ControlMode::Keyboard
    };
    for (mut text, check_box) in check_box_query.iter_mut() {
        if check_box.is_click_mode {
            text.sections[0].value = if *is_click_mode {
                String::from("-> ")
            } else {
                String::from("")
            };
        } else {
            text.sections[0].value = if *is_click_mode {
                String::from("")
            } else {
                String::from("-> ")
            };
        }
    }
}

fn handle_start_button_interaction(
    mut commands: Commands,
    mut start_button_query: Query<(&Interaction, &mut BackgroundColor), With<StartButton>>,
    mut window_query: Query<&mut Window>,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut window = window_query.get_single_mut().unwrap();
    let (interaction, mut background) = start_button_query.get_single_mut().unwrap();
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
            let main_menu = main_menu_query.get_single().unwrap();
            commands.entity(main_menu).despawn_recursive();
            next_state.set(AppState::Game)
        }
    }
}

fn start_tips_animation(mut start_tip_queries: Query<(&mut Text, &mut StartTips)>) {
    let (mut text, mut start_tips) = start_tip_queries.get_single_mut().unwrap();
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
