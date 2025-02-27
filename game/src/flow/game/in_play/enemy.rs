use bevy::prelude::*;
use rand::{rng, Rng};
use shooting_game_shared::util::{EdgeUtil, UFO_SIZE};

use crate::components::{Player, Score, Velocity, UFO};
use crate::states::GameState;
use shooting_game_shared::game_related::Stage;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_and_spawn_enemy,
                handle_horizontal_movement,
                cleanup_on_out_screen,
            )
                .run_if(in_state(GameState::InPlay)),
        );
    }
}

fn handle_horizontal_movement(mut ufo_query: Query<(&mut Velocity, &Transform), With<UFO>>) {
    let edge = EdgeUtil::new(UFO_SIZE);
    for (mut velocity, transform) in ufo_query.iter_mut() {
        let x = transform.translation.x;
        if edge.over_left_in(x) || edge.over_right_in(x) {
            velocity.x = -velocity.x;
        }
    }
}

fn check_and_spawn_enemy(
    commands: Commands,
    ufo_query: Query<Entity, With<UFO>>,
    score_query: Query<&Score, With<Player>>,
) {
    let ufo_number = ufo_query.iter().len();
    let Ok(score) = score_query.get_single() else {
        warn!("Should have exactly one player");
        return;
    };
    let stage = Stage::new(score.0);
    if ufo_number == 0 || stage.random_generator(ufo_number) {
        spawn_ufo(commands, Velocity::from_vec2(stage.get_ufo_velocity()));
    }
}

fn spawn_ufo(mut commands: Commands, velocity: Velocity) {
    let mut rng = rng();
    let edge = EdgeUtil::ufo();
    let ufo_position = Vec2::new(
        rng.random_range(edge.left_in()..edge.right_in()),
        edge.top_out(),
    );
    commands.spawn((UFO::new(ufo_position), velocity));
}

fn cleanup_on_out_screen(
    mut commands: Commands,
    ufo_query: Query<(Entity, &Transform), With<UFO>>,
) {
    let edge = EdgeUtil::new(UFO_SIZE);
    for (entity, transform) in ufo_query.iter() {
        if edge.over_bottom_out(transform.translation.y) {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
    }
}
