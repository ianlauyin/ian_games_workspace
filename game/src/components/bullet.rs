use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;

use crate::{
    constant::{ZIndex, BULLET_SIZE},
    util::EdgeUtil,
};

use super::{Player, Spaceship, Velocity};

#[derive(Component)]
pub struct Bullet {
    player: u8,
}

impl Bullet {
    pub fn by_player(player: u8) -> Self {
        Self { player }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cleanup_on_out_screen)
            .add_observer(bullet_on_added);
    }
}

fn bullet_on_added(
    ev: Trigger<OnAdd, Bullet>,
    mut commands: Commands,
    bullet_q: Query<&Bullet>,
    spaceship_q: Query<(&Player, &Spaceship)>,
) {
    let bullet = bullet_q.get(ev.entity()).unwrap();
    for (player, spaceship) in spaceship_q.iter() {
        if player.0 == bullet.player {
            commands.entity(ev.entity()).insert((
                Velocity { x: 0., y: 10. },
                Transform::from_translation(
                    spaceship.get_position().extend(ZIndex::BULLET.z_value()),
                ),
                Sprite {
                    color: Color::from(YELLOW),
                    custom_size: Some(BULLET_SIZE),
                    ..default()
                },
            ));
        }
    }
}

fn cleanup_on_out_screen(
    mut commands: Commands,
    bullet_queries: Query<(Entity, &Transform), With<Bullet>>,
) {
    let edge = EdgeUtil::new(BULLET_SIZE);
    for (entity, transform) in bullet_queries.iter() {
        if edge.over_top_out(transform.translation.y) {
            commands.entity(entity).despawn();
        }
    }
}
