use bevy::prelude::*;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    #[default]
    LoadAsset,
    MainMenu,
    InPlay,
}

#[derive(Default, SubStates, Debug, Hash, Eq, PartialEq, Clone)]
#[source(AppState = AppState::InPlay)]
pub enum GameState {
    #[default]
    Ready,
    InPlay,
    Result,
}
