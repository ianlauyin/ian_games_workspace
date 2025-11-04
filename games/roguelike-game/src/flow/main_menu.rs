use bevy::prelude::*;

use crate::state::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), || {
            println!("MainMenu");
        });
    }
}
