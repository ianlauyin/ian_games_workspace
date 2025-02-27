use bevy::prelude::*;

use crate::components::{Bullet, Player, Spaceship};

#[derive(Event)]
pub struct UpdatePositionEvent {
    pub player_tag: u8,
    pub position: Vec2,
    pub bullets: Vec<(f32, f32)>,
}

pub struct UpdatePositionPlugin;

impl Plugin for UpdatePositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_position);
    }
}

fn update_position(
    trigger: Trigger<UpdatePositionEvent>,
    mut commands: Commands,
    mut spaceships: Query<(&mut Transform, &Player), With<Spaceship>>,
    bullets: Query<(Entity, &Player), With<Bullet>>,
) {
    let ev = trigger.event();
    for (mut transform, player) in spaceships.iter_mut() {
        if player.0 == ev.player_tag {
            transform.translation.x = ev.position.x;
            transform.translation.y = ev.position.y;
            break;
        }
    }
    for (entity, player) in bullets.iter() {
        if player.0 == ev.player_tag {
            commands.entity(entity).despawn_recursive();
        }
    }
    for bullet in ev.bullets.iter() {
        commands.spawn(Bullet::by_player(
            ev.player_tag,
            Vec2::new(bullet.0, bullet.1),
        ));
    }
}
