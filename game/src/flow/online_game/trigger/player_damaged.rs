use bevy::prelude::*;

use crate::components::{Health, Invisible, Player, Spaceship};

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
    ev: Trigger<PlayerDamagedEvent>,
    mut commands: Commands,
    spaceship_q: Query<(Entity, &Player), With<Spaceship>>,
    mut health_q: Query<(&mut Health, &Player)>,
) {
    let event = ev.event();
    for (mut health, player) in health_q.iter_mut() {
        if player.0 == event.tag {
            health.0 = event.new_health;
            break;
        }
    }
    for (entity, player) in spaceship_q.iter() {
        if player.0 == event.tag {
            commands.entity(entity).insert(Invisible::new());
            return;
        }
    }
}
