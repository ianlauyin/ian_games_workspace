mod collision;
mod control;
mod enemy;
mod finish;
mod health_display;
mod score_display;
mod shooting;

use bevy::prelude::*;
pub struct InPlayPlugin;

impl Plugin for InPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            control::ControlPlugin,
            health_display::HealthDisplayPlugin,
            score_display::ScoreDisplayPlugin,
            enemy::EnemyPlugin,
            collision::CollisionPlugin,
            finish::FinishPlugin,
            shooting::ShootingPlugin,
        ));
    }
}
