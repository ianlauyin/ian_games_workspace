use crate::{
    components::{Bullet, CollidedEvent, SelfPlayer, Spaceship, UFO},
    flow::online_game::connection::SendMessageEvent,
    states::OnlineGameState,
};
use bevy::prelude::*;
use shooting_game_shared::ClientMessage;

use super::EnemyTag;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_collisions.run_if(in_state(OnlineGameState::InPlay)),
        );
    }
}

pub fn handle_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollidedEvent>,
    enemy_tag_q: Query<&EnemyTag, With<UFO>>,
    spaceship_q: Query<Entity, (With<Spaceship>, With<SelfPlayer>)>,
    bullet_q: Query<&Bullet>,
) {
    for collision in collision_events.read() {
        let Ok(enemy_tag) = enemy_tag_q.get(collision.enemy) else {
            continue;
        };
        let player_entity = collision.player;

        if let Ok(_) = spaceship_q.get(player_entity) {
            commands.trigger(SendMessageEvent(ClientMessage::DamagedIntent {
                enemy_tag: enemy_tag.0,
            }));
        }

        // bullet-ufo collision
        // if let Ok(bullet) = bullet_q.get(player_entity) {
        //     return handle_bullet_ufo_collision(
        //         commands.reborrow(),
        //         bullet,
        //         player_entity,
        //         ufo,
        //         collision.enemy,
        //     );
        // }
    }
}

// fn handle_bullet_ufo_collision(
//     mut commands: Commands,
//     bullet: &Bullet,
//     bullet_entity: Entity,
//     ufo: &UFO,
//     ufo_entity: Entity,
// ) {
//     if let Some(mut entity_commands) = commands.get_entity(bullet_entity) {
//         entity_commands.despawn();
//     }
//     commands.trigger(RemoveUFOEvent::by_player(ufo_entity, bullet.get_player()));
//     commands.spawn(Explosion::new(ufo.get_position()));
// }
