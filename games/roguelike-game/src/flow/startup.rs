use bevy::prelude::*;
use bevy::window::WindowResolution;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::from((640, 480)),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(games_lib::DevtoolPlugin)
        .add_systems(PreStartup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
