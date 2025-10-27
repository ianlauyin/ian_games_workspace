mod collision;
mod display;
mod from_server;
mod out_screen_cleanup;
mod enemy;
use bevy::prelude::*;

pub struct InPlayPlugin;

impl Plugin for InPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            display::DisplayPlugin,
            collision::CollisionPlugin,
            from_server::FromServerPlugin,
            out_screen_cleanup::OutScreenCleanupPlugin,
            enemy::EnemyPlugin,
        ));
    }
}
