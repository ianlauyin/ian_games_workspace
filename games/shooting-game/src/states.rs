use bevy::prelude::*;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_sub_state::<GameState>();
    }
}

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
