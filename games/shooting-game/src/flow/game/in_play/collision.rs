use crate::{
    components::{Bullet, CollidedEvent, Explosion, Invisible, Player, Spaceship, UFO},
    flow::game::triggers::{HealthReduceEvent, RemoveUFOEvent},
    states::GameState,
    util::Position,
};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_collisions.run_if(in_state(GameState::InPlay)),
        );
    }
}

pub fn handle_collisions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollidedEvent>,
    ufo_q: Query<&UFO>,
    spaceship_q: Query<&Player, With<Spaceship>>,
    bullet_q: Query<&Bullet>,
) {
    for collision in collision_events.read() {
        let Ok(ufo) = ufo_q.get(collision.enemy) else {
            continue;
        };
        let player_entity = collision.player;

        if let Ok(player) = spaceship_q.get(player_entity) {
            return handle_ufo_spaceship_collision(
                commands.reborrow(),
                player,
                player_entity,
                ufo,
                collision.enemy,
            );
        }

        // bullet-ufo collision
        if let Ok(bullet) = bullet_q.get(player_entity) {
            return handle_bullet_ufo_collision(
                commands.reborrow(),
                bullet,
                player_entity,
                ufo,
                collision.enemy,
            );
        }
    }
}

fn handle_ufo_spaceship_collision(
    mut commands: Commands,
    player: &Player,
    player_entity: Entity,
    ufo: &UFO,
    ufo_entity: Entity,
) {
    commands.trigger(HealthReduceEvent::new(player.0));
    if let Ok(mut entity_commands) = commands.get_entity(player_entity) {
        entity_commands.insert(Invisible::new());
    }
    commands.spawn(Explosion::new(ufo.get_position()));
    commands.trigger(RemoveUFOEvent::clean_up(ufo_entity));
}

fn handle_bullet_ufo_collision(
    mut commands: Commands,
    bullet: &Bullet,
    bullet_entity: Entity,
    ufo: &UFO,
    ufo_entity: Entity,
) {
    if let Ok(mut entity_commands) = commands.get_entity(bullet_entity) {
        entity_commands.despawn();
    }
    commands.trigger(RemoveUFOEvent::by_player(ufo_entity, bullet.get_player()));
    commands.spawn(Explosion::new(ufo.get_position()));
}
