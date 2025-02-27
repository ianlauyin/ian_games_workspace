use bevy::prelude::*;

use crate::components::{Bullet, Player, UFO};
use crate::res::PlayerTag;
use crate::{states::AppState, util::cleanup_components};

pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            (
                cleanup_components::<Player>,
                cleanup_components::<Bullet>,
                cleanup_components::<UFO>,
                reset_player_tag,
            ),
        );
    }
}

fn reset_player_tag(mut player_tag: ResMut<PlayerTag>) {
    player_tag.0 = 1;
}
