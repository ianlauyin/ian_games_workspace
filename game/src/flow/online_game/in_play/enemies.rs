use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyTag(pub u128);

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {}
}
