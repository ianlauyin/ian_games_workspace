use bevy::prelude::*;

use crate::components::{Bullet, Player, UFO};
use crate::{states::AppState, util::cleanup_components};

pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                exited: AppState::Game,
                entered: AppState::MainMenu,
            },
            (
                cleanup_components::<Player>,
                cleanup_components::<Bullet>,
                cleanup_components::<UFO>,
            ),
        );
    }
}
