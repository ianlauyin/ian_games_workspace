use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::constant::{ZIndexMap, MOBILE_WINDOW_SIZE};
use crate::states::{AppState, GameState};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#shooting-game".into()),
                fit_canvas_to_parent: true,
                resolution: WindowResolution::from(MOBILE_WINDOW_SIZE),
                ..default()
            }),
            ..default()
        }))
        .add_systems(OnExit(AppState::Loading), (setup_camera, setup_background));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_background(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.05, 0., 0.05),
            custom_size: Some(MOBILE_WINDOW_SIZE),
            ..default()
        },
        Transform::from_xyz(0., 0., ZIndexMap::BACKGROUND),
    ));
}
