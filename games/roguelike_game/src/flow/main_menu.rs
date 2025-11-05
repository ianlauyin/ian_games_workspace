use bevy::prelude::*;
use game_lib::component::OverlayNode;
use game_lib::system::cleanup_components;

use crate::state::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_components::<MainMenu>);
    }
}

#[derive(Component)]
struct MainMenu;

fn spawn_main_menu(mut commands: Commands) {
    let start_button = commands
        .spawn((
            Button,
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Px(100.),
                height: Val::Px(50.),
                ..default()
            },
            BorderRadius::all(Val::Px(5.)),
            BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 1.)),
        ))
        .with_child(Text::new("Start"))
        .id();

    let main_menu_container = commands
        .spawn(Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            ..default()
        })
        .add_child(start_button)
        .id();

    commands
        .spawn((MainMenu, OverlayNode))
        .add_child(main_menu_container);
}
