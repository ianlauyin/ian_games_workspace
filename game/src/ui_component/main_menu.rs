use bevy::app::App;
use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

use crate::control::{ControlMode, ControlOption};
use crate::states::AppState;
use crate::ui_component::ZIndexMap;

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
            .add_observer(change_control);
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
    let is_click_mode = control_option.mode == ControlMode::Button;
    commands
        .spawn((
            MainMenu,
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                padding: UiRect::all(Val::Px(10.)),
                row_gap: Val::Px(5.),
                ..default()
            },
            BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 0.5)),
            Transform {
                translation: Vec3::default().with_z(ZIndexMap::MainMenu.value()),
                ..default()
            },
        ))
        .with_children(|menu_background| {
            menu_background.spawn((
                Text::new("Whenever the ufo crash you, you will lose health."),
                Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
            ));
            menu_background.spawn((
                Text::new("Every 50 score will increase the difficulty"),
                Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
            ));
            menu_background.spawn((
                Node {
                    margin: UiRect::top(Val::Px(50.)),
                    ..default()
                },
                Text::new("In KeyBoard Mode:"),
                TextColor(Color::srgba(0., 0., 1., 1.)),
                Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
            ));
            menu_background.spawn((
                Text::new("Press Arrow move"),
                Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
            ));

            menu_background.spawn((
                Text::new("Press Space to shoot bullet"),
                Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
            ));
            menu_background.spawn((
                Node {
                    margin: UiRect::top(Val::Px(50.)),
                    ..default()
                },
                Text::new("In Button Mode:"),
                TextColor(Color::srgba(0., 1., 0., 1.)),
                Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
            ));

            menu_background.spawn((
                Text::new("Hover on Arrow to move"),
                Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
            ));
            menu_background.spawn((
                Text::new("Bullet will shoot automatically"),
                Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
            ));
            menu_background
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    row_gap: Val::Px(10.),
                    flex_grow: 1.,
                    ..default()
                })
                .with_children(|option_node| {
                    option_node
                        .spawn((
                            CheckBox {
                                is_click_mode: false,
                            },
                            Interaction::default(),
                            TextLayout::new_with_justify(JustifyText::Right),
                            Text::new(if is_click_mode { "" } else { "-> " }),
                        ))
                        .with_child((
                            TextSpan::new("Use Keyboard to play"),
                            TextColor(Color::srgba(0., 0., 1., 1.)),
                        ));
                    option_node
                        .spawn((
                            CheckBox {
                                is_click_mode: true,
                            },
                            Interaction::default(),
                            TextLayout::new_with_justify(JustifyText::Right),
                            Text::new(if is_click_mode { "-> " } else { "" }),
                        ))
                        .with_child((
                            TextSpan::new("Use Keyboard to play"),
                            TextColor(Color::srgba(0., 1., 0., 1.)),
                        ));
                    option_node.spawn((
                        StartTips { appearing: false },
                        TextLayout::new_with_justify(JustifyText::Center),
                        Text::new("Click Start to start the game"),
                        Transform {
                            translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                            ..default()
                        },
                    ));
                    option_node
                        .spawn((
                            StartButton,
                            Interaction::default(),
                            Node {
                                align_self: AlignSelf::FlexEnd,
                                width: Val::Px(100.),
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
                        .with_child(Text::new("Start"));
                });
        });
}

fn handle_check_box_interaction(
    mut commands: Commands,
    mut check_box_query: Query<(&Interaction, &mut TextColor, &CheckBox)>,
) {
    for (interaction, mut text_color, check_box) in check_box_query.iter_mut() {
        match interaction {
            Interaction::Hovered => text_color.set_alpha(0.5),
            Interaction::None => text_color.set_alpha(1.0),
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
    let current_is_click_mode = control_option.mode == ControlMode::Button;
    if *is_click_mode == current_is_click_mode {
        return;
    }
    control_option.mode = if *is_click_mode {
        ControlMode::Button
    } else {
        ControlMode::Keyboard
    };
    for (mut text, check_box) in check_box_query.iter_mut() {
        if check_box.is_click_mode {
            text.0 = if *is_click_mode {
                String::from("-> ")
            } else {
                String::from("")
            };
        } else {
            text.0 = if *is_click_mode {
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
    mut window_query: Query<Entity, With<Window>>,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let window_entity = window_query.get_single_mut().unwrap();
    let (interaction, mut background) = start_button_query.get_single_mut().unwrap();
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
            let main_menu = main_menu_query.get_single().unwrap();
            commands.entity(main_menu).despawn_recursive();
            next_state.set(AppState::Game)
        }
    }
}

fn start_tips_animation(mut start_tip_queries: Query<(&mut TextColor, &mut StartTips)>) {
    let (mut text_color, mut start_tips) = start_tip_queries.get_single_mut().unwrap();
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
