use bevy::prelude::*;

use crate::{states::OnlineGameState, ui_components::MainContainer};

pub struct MatchingPlugin;

impl Plugin for MatchingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OnlineGameState::Matching), setup_matching);
    }
}

fn setup_matching(mut commands: Commands) {
    commands
        .spawn(MainContainer)
        .with_child(Text::new("Matching"));
}