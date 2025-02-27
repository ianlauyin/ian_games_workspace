mod spawn_enemy;
mod update_position;

use bevy::prelude::*;

pub use spawn_enemy::SpawnEnemyEvent;
pub use update_position::UpdatePositionEvent;
pub struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            update_position::UpdatePositionPlugin,
            spawn_enemy::SpawnEnemyPlugin,
        ));
    }
}
