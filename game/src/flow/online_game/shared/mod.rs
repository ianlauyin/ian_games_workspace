mod send_player_info;
mod update_player_info;

use bevy::prelude::*;
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            send_player_info::SendPlayerInfoPlugin,
            update_player_info::UpdatePlayerInfoPlugin,
        ));
    }
}
