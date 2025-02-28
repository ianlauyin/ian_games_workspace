use crate::{
    components::{BulletTag, CollidedEvent, EnemyTag, SelfPlayer, Spaceship, UFO},
    flow::online_game::connection::SendMessageEvent,
    states::OnlineGameState,
};
use bevy::prelude::*;
use shooting_game_shared::ClientMessage;

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
    bullet_q: Query<&BulletTag>,
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

        if let Ok(bullet_tag) = bullet_q.get(player_entity) {
            commands.trigger(SendMessageEvent(ClientMessage::DestroyEnemyIntent {
                bullet_tag: bullet_tag.0,
                enemy_tag: enemy_tag.0,
            }));
        }
    }
}
