use bevy::prelude::*;

use super::super::in_play::EnemyTag;
use crate::components::{Velocity, UFO};

#[derive(Event)]
pub struct SpawnEnemyEvent {
    pub tag: u128,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
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
        UFO::new(Vec2::new(enemy.position.0, enemy.position.1)),
        EnemyTag(enemy.tag),
        Velocity {
            x: enemy.velocity.0,
            y: enemy.velocity.1,
        },
    ));
}
