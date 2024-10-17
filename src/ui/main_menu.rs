use bevy::app::App;
use bevy::prelude::*;

use crate::states::AppState;
use crate::ui::ZIndexMap;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), show_main_menu)
            .add_systems(Update, handle_play.run_if(in_state(AppState::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct StartTips;
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
                text: Text::from_section("Press Arrow Left <- to move left", TextStyle::default()),
                transform: Transform {
                    translation: Vec3::default().with_z(ZIndexMap::Text.value()),
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Press Arrow Right -> to move right",
                    TextStyle::default(),
                ),
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
                StartTips,
                TextBundle {
                    style: Style {
                        margin: UiRect::top(Val::Percent(50.)),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section("Press Space to start the game", TextStyle::default()),
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
    if keys.just_pressed(KeyCode::Space) {
        let main_menu = main_menu_query.get_single().unwrap();
        commands.entity(main_menu).despawn_recursive();
        next_state.set(AppState::Game)
    }
}

fn start_tip_animation(mut start_tip_queries: Query<&mut Text, With<StartTips>>) {}
