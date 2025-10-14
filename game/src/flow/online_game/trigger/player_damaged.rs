use bevy::prelude::*;

use crate::{
    components::{Explosion, Health, Invisible, Player, Spaceship},
    util::Position,
};

#[derive(Event)]
pub struct PlayerDamagedEvent {
    tag: u8,
    new_health: u8,
}

impl PlayerDamagedEvent {
    pub fn update_health(tag: u8, new_health: u8) -> Self {
        Self { tag, new_health }
    }
}

pub struct PlayerDamagedPlugin;

impl Plugin for PlayerDamagedPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(player_damaged);
    }
}

fn player_damaged(
    ev: On<PlayerDamagedEvent>,
    mut commands: Commands,
    spaceship_q: Query<(Entity, &Player, &Spaceship)>,
    mut health_q: Query<(&mut Health, &Player)>,
) {
    let event = ev.event();
    for (mut health, player) in health_q.iter_mut() {
        if player.0 == event.tag {
            health.0 = event.new_health;
            break;
        }
    }
    for (entity, player, spaceship) in spaceship_q.iter() {
        if player.0 == event.tag {
            if event.new_health == 0 {
                commands.spawn(Explosion::new(spaceship.get_position()));
                commands.entity(entity).despawn();
            } else {
                commands.entity(entity).insert(Invisible::new());
            }
            return;
        }
    }
}
