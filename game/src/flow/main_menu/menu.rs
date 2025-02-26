use bevy::app::App;
use bevy::prelude::*;

use crate::res::{ControlMode, ControlOption};
use crate::states::AppState;
use crate::ui_components::{Blink, InteractionUI, MainContainer, SelectableText};
use crate::util::cleanup_components;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), show_main_menu)
            .add_systems(
                Update,
                (
                    (
                        handle_control_mode_selection,
                        handle_control_mode_selection_text,
                    )
                        .chain(),
                    handle_start_button_interaction,
                )
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_components::<MainMenu>);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum StartButton {
    Game,
    OnlineGame,
}

fn show_main_menu(mut commands: Commands, control_option: Res<ControlOption>) {
    let is_keyboard_mode = control_option.mode == ControlMode::Keyboard;
    commands
        .spawn((MainMenu, MainContainer))
        .with_children(|menu_background| {
            menu_background.spawn(Text::new(
                "Whenever the ufo crash you, you will lose health.\nEvery 50 score will increase the difficulty",
            ));
            menu_background.spawn((
                Node {
                    margin: UiRect::top(Val::Px(50.)),
                    ..default()
                },
                Text::new("In KeyBoard Mode:"),
                TextColor(Color::srgba(0., 0., 1., 1.)),
            ));
            menu_background.spawn(Text::new("Press Arrow move\nPress Space to shoot bullet"));

            menu_background.spawn((
                Node {
                    margin: UiRect::top(Val::Px(50.)),
                    ..default()
                },
                Text::new("In Button Mode:"),
                TextColor(Color::srgba(0., 1., 0., 1.)),
            ));
            menu_background.spawn(Text::new("Hover on Arrow to move\nBullet will shoot automatically"));

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
                            ControlMode::Keyboard,
                            SelectableText::new("Use Keyboard Mode to play",is_keyboard_mode),
                            Interaction::default(),
                            TextLayout::new_with_justify(JustifyText::Right),
                            TextColor(Color::srgba(0., 0., 1., 1.)),
                        ));
                    option_node
                        .spawn((
                            ControlMode::Button,
                            SelectableText::new("Use Button Mode to play",!is_keyboard_mode),
                            Interaction::default(),
                            TextLayout::new_with_justify(JustifyText::Right),
                            TextColor(Color::srgba(0., 1., 0., 1.)),
                        ));
                    option_node.spawn((
                        Blink::new_with_speed(0.02),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Text::new("Click Start to start the game"),
                    ));
                    option_node
                        .spawn((
                            StartButton::Game,
                            InteractionUI,
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
                            BorderRadius::all(Val::Px(5.))
                        ))
                        .with_child(Text::new("Start"));
                    option_node
                    .spawn((
                        StartButton::OnlineGame,
                        InteractionUI,
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
                        BorderRadius::all(Val::Px(5.))
                    ))
                    .with_child(Text::new("Online Game"));
                });
        });
}

fn handle_control_mode_selection(
    control_mode_query: Query<(&ControlMode, &Interaction)>,
    mut control_option: ResMut<ControlOption>,
) {
    for (control_mode, interaction) in control_mode_query.iter() {
        if *interaction == Interaction::Pressed {
            control_option.set_mode(control_mode);
        }
    }
}

fn handle_control_mode_selection_text(
    mut control_mode_query: Query<(&ControlMode, &mut SelectableText)>,
    control_option: Res<ControlOption>,
) {
    if control_option.is_changed() {
        for (control_mode, mut selectable_text) in control_mode_query.iter_mut() {
            selectable_text.set_selected(*control_mode == control_option.mode);
        }
    }
}

fn handle_start_button_interaction(
    mut commands: Commands,
    start_button_query: Query<(&Interaction, &StartButton)>,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, start_button) in start_button_query.iter() {
        if *interaction == Interaction::Pressed {
            let Ok(main_menu) = main_menu_query.get_single() else {
                panic!("Main Menu not found in handle_start_button_interaction");
            };
            if let Some(entity_commands) = commands.get_entity(main_menu) {
                entity_commands.despawn_recursive();
            }
            let target_state = match start_button {
                StartButton::Game => AppState::Game,
                StartButton::OnlineGame => AppState::OnlineGame,
            };
            next_state.set(target_state);
        };
    }
}
