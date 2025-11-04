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
    commands.spawn((MainMenu, OverlayNode));
}
