use bevy::prelude::*;
use shooting_game_util::EdgeUtil;

use crate::{
    components::{Bullet, Player, Spaceship},
    constant::BULLET_SIZE,
    res::{ControlMode, ControlOption},
    states::GameState,
};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shooting_bullet, cleanup_on_out_screen).run_if(in_state(GameState::InPlay)),
        );
    }
}

fn shooting_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    control_option: Res<ControlOption>,
    mut spaceship_query: Query<(&mut Spaceship, &Player)>,
) {
    if keys.pressed(KeyCode::Space) || control_option.mode == ControlMode::Button {
        let Ok((mut spaceship, player)) = spaceship_query.get_single_mut() else {
            return;
        };
        if spaceship.can_shoot() {
            commands.spawn(Bullet::by_player(player.0));
            spaceship.start_cd();
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
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
    }
}
