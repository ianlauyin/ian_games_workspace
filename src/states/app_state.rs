use bevy::prelude::States;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    #[default]
    LoadAsset,
    MainMenu,
    InPlay,
    Result,
}
