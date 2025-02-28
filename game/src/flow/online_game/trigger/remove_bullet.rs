use bevy::prelude::*;

use crate::components::BulletTag;

#[derive(Event)]
pub struct RemoveBulletEvent(pub u16);

pub struct RemoveBulletPlugin;

impl Plugin for RemoveBulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(remove_bullet);
    }
}

fn remove_bullet(
    ev: Trigger<RemoveBulletEvent>,
    mut commands: Commands,
    bullet_q: Query<(Entity, &BulletTag)>,
) {
    let event = ev.event();
    for (entity, bullet_tag) in bullet_q.iter() {
        if bullet_tag.0 == event.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
