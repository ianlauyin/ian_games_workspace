mod enemies;

use bevy::prelude::*;
pub use enemies::EnemyTag;

pub struct InPlayPlugin;

impl Plugin for InPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((enemies::EnemiesPlugin,));
    }
}
