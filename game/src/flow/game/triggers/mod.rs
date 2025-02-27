mod add_score;
mod health_reduce;
mod remove_ufo;

pub use add_score::AddScoreEvent;
pub use health_reduce::HealthReduceEvent;
pub use remove_ufo::RemoveUFOEvent;

use bevy::prelude::{App, Plugin};

pub struct TriggersPlugin;

impl Plugin for TriggersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            remove_ufo::RemoveUFOPlugin,
            add_score::AddScorePlugin,
            health_reduce::HealthReducePlugin,
        ));
    }
}
