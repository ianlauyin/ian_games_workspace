use bevy::app::App;
use bevy::prelude::*;

use crate::states::AppState;
use crate::ui::ZIndexMap;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), show_main_menu)
            .add_systems(
                Update,
                (start_tips_animation, handle_play).run_if(in_state(AppState::MainMenu)),
            );
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct StartTips {
    appearing: bool,
}
fn show_main_menu(mut commands: Commands) {
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
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Press Arrow move", TextStyle::default()),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section("Press Space to shoot bullet", TextStyle::default()),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
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
            parent.spawn(TextBundle {
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
            parent.spawn((
                StartTips { appearing: false },
                TextBundle {
                    style: Style {
                        margin: UiRect::top(Val::Percent(50.)),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section("Press Enter to start the game", TextStyle::default()),
                    transform: Transform {
                        translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn handle_play(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        let main_menu = main_menu_query.get_single().unwrap();
        commands.entity(main_menu).despawn_recursive();
        next_state.set(AppState::Game)
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
