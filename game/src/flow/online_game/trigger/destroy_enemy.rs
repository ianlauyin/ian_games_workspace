use bevy::prelude::*;

use crate::{
    components::{EnemyTag, Explosion, UFO},
    util::Position,
};

#[derive(Event)]
pub struct DestroyEnemyEvent(pub u16);

pub struct DestroyEnemyPlugin;

impl Plugin for DestroyEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(remove_enemy);
    }
}

fn remove_enemy(
    ev: On<DestroyEnemyEvent>,
    mut commands: Commands,
    enemy_q: Query<(Entity, &UFO, &EnemyTag)>,
) {
    let remove_enemy_tag = ev.event().0;
    for (enemy, ufo, enemy_tag) in enemy_q.iter() {
        if enemy_tag.0 == remove_enemy_tag {
            commands.spawn(Explosion::new(ufo.get_position()));
            commands.entity(enemy).despawn();
            return;
        }
    }
}
