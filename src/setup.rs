use bevy::app::{App, Startup};
use bevy::prelude::{AppExtStates, Camera2dBundle, Commands, Plugin};

use crate::states::AppState;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(Startup, setup_camera);
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
