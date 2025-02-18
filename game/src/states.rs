use bevy::prelude::*;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Game,
}

#[derive(Default, SubStates, Debug, Hash, Eq, PartialEq, Clone)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    #[default]
    Ready,
    InPlay,
    Result,
}
