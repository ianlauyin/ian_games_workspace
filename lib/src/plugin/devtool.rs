use bevy::camera::RenderTarget;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::window::WindowRef;
use bevy::window::WindowResolution;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::bevy_egui::EguiPrimaryContextPass;
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::egui;

const DEFAULT_DEVTOOL_WINDOW_SIZE: (f32, f32) = (320., 600.);

// Add a secondary window for debugging
pub struct DevtoolPlugin;

impl Plugin for DevtoolPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugins(EguiPlugin::default())
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_systems(PostStartup, spawn_devtool_window)
            .add_observer(kill_app_when_window_closed)
            .add_systems(EguiPrimaryContextPass, world_inspector_ui);
    }
}

#[derive(Component)]
struct DevtoolWindow;

#[derive(Component)]
struct DevtoolCamera;

fn spawn_devtool_window(mut commands: Commands) {
    let devtool_window = commands
        .spawn((
            DevtoolWindow,
            Window {
                title: "Devtool".to_string(),
                resolution: WindowResolution::new(
                    DEFAULT_DEVTOOL_WINDOW_SIZE.0 as u32,
                    DEFAULT_DEVTOOL_WINDOW_SIZE.1 as u32,
                ),
                ..default()
            },
        ))
        .id();
    commands.spawn((
        DevtoolCamera,
        RenderLayers::layer(1),
        Camera2d,
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(devtool_window)),
            ..default()
        },
    ));
}

fn kill_app_when_window_closed(_: On<Remove, Window>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

fn world_inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<DevtoolCamera>>()
        .single(world)
    else {
        warn!("No DevtoolCamera found in world_inspector_ui");
        return;
    };

    let mut egui_context = egui_context.clone();

    let Ok(devtool_window) = world
        .query_filtered::<&mut Window, With<DevtoolWindow>>()
        .single(world)
    else {
        warn!("No DevtoolWindow found in world_inspector_ui");
        return;
    };
    let devtool_window = devtool_window.clone();

    egui::Window::new("World Inspector")
        .default_size(DEFAULT_DEVTOOL_WINDOW_SIZE)
        .resizable(false)
        .collapsible(false)
        .movable(false)
        .show(egui_context.get_mut(), |window_ui| {
            egui::ScrollArea::both().show(window_ui, |scroll_area_ui| {
                bevy_inspector::ui_for_world(world, scroll_area_ui);
                scroll_area_ui.allocate_space(scroll_area_ui.available_size());
            });
            window_ui.set_height(devtool_window.resolution.height());
            window_ui.set_width(devtool_window.resolution.width());
        });
}
