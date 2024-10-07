use bevy::prelude::States;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    Ready,
    InPlay,
    Result,
}
