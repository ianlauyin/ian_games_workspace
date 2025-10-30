mod startup;

pub struct FlowPlugin;

impl bevy::app::Plugin for FlowPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(startup::StartupPlugin);
    }
}
