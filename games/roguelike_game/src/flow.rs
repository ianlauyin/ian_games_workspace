mod main_menu;
pub struct FlowPlugin;

impl bevy::app::Plugin for FlowPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(main_menu::MainMenuPlugin);
    }
}
