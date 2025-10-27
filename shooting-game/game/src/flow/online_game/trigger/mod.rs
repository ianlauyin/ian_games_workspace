mod add_score;
mod destroy_enemy;
mod player_damaged;
mod remove_bullet;
mod spawn_enemy;
mod update_position;

use bevy::prelude::*;

pub use add_score::AddScoreEvent;
pub use destroy_enemy::DestroyEnemyEvent;
pub use player_damaged::PlayerDamagedEvent;
pub use remove_bullet::RemoveBulletEvent;
pub use spawn_enemy::SpawnEnemyEvent;
pub use update_position::UpdatePositionEvent;
pub struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            update_position::UpdatePositionPlugin,
            spawn_enemy::SpawnEnemyPlugin,
            destroy_enemy::DestroyEnemyPlugin,
            player_damaged::PlayerDamagedPlugin,
            add_score::AddScorePlugin,
            remove_bullet::RemoveBulletPlugin,
        ));
    }
}
