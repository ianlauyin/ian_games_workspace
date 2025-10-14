use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use super::invisible::Invisible;

#[derive(Component)]
#[require(Sprite)]
pub enum Collisable {
    Enemy,
    Player,
}

#[derive(Message)]
pub struct CollidedEvent {
    pub player: Entity,
    pub enemy: Entity,
}

pub struct CollisablePlugin;

impl Plugin for CollisablePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CollidedEvent>()
            .add_systems(Update, check_collision);
    }
}

fn check_collision(
    mut event_writer: MessageWriter<CollidedEvent>,
    collisable_query: Query<(Entity, &Transform, &Sprite, &Collisable), Without<Invisible>>,
) {
    let mut players: Vec<(Entity, Aabb2d)> = Vec::new();
    let mut enemies: Vec<(Entity, Aabb2d)> = Vec::new();

    for (entity, transform, sprite, collisable) in collisable_query.iter() {
        let aabb = Aabb2d::new(
            transform.translation.truncate(),
            sprite.custom_size.unwrap() / 2.,
        );

        match collisable {
            Collisable::Player => players.push((entity, aabb)),
            Collisable::Enemy => enemies.push((entity, aabb)),
        }
    }

    for (player_entity, player_aabb) in players.iter() {
        for (enemy_entity, enemy_aabb) in enemies.iter() {
            if player_aabb.intersects(enemy_aabb) {
                event_writer.write(CollidedEvent {
                    player: *player_entity,
                    enemy: *enemy_entity,
                });
                return;
            }
        }
    }
}
