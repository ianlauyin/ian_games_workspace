use bevy::prelude::*;

use crate::components::{EnemyTag, Velocity, UFO};

#[derive(Event)]
pub struct SpawnEnemyEvent {
    pub tag: u16,
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct SpawnEnemyPlugin;

impl Plugin for SpawnEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_enemy);
    }
}

fn spawn_enemy(ev: Trigger<SpawnEnemyEvent>, mut commands: Commands) {
    let enemy = ev.event();
    commands.spawn((
        UFO::new(enemy.position),
        EnemyTag(enemy.tag),
        Velocity::from_vec2(enemy.velocity),
    ));
}
