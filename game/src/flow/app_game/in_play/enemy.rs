use bevy::prelude::*;
use rand::{rng, Rng};

use crate::{
    constant::UFO_SIZE,
    game_component::{Player, Score, UFO},
    states::GameState,
    ui_component::Velocity,
    util::EdgeUtil,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_and_spawn_enemy,
                handle_horizontal_movement,
                ufo_cleanup,
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

fn ufo_cleanup(mut commands: Commands, ufo_query: Query<(Entity, &Transform), With<UFO>>) {
    let edge = EdgeUtil::new(UFO_SIZE);
    for (entity, transform) in ufo_query.iter() {
        if edge.over_bottom_out(transform.translation.y) {
            commands.entity(entity).despawn_recursive();
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
    if ufo_number == 0 || stage.random_generator(ufo_number as f64) {
        spawn_ufo(commands, Velocity::from_vec2(stage.get_ufo_velocity()));
    }
}

fn spawn_ufo(mut commands: Commands, velocity: Velocity) {
    let mut rng = rng();
    let edge = EdgeUtil::new(UFO_SIZE);
    let ufo_position = Vec2::new(
        rng.random_range(edge.left_in()..edge.right_in()),
        edge.top_out(),
    );
    commands.spawn((UFO::new(ufo_position), velocity));
}
enum Stage {
    Warmup,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Stage {
    fn new(score: u8) -> Stage {
        match score {
            0..10 => Stage::Warmup,
            10..50 => Stage::One,
            50..100 => Stage::Two,
            100..150 => Stage::Three,
            150..200 => Stage::Four,
            200..250 => Stage::Five,
            _ => Stage::Six,
        }
    }

    fn random_generator(&self, existing_ufo: f64) -> bool {
        let mut rng = rng();
        return match self {
            Stage::Warmup => rng.random_bool(0.01),
            Stage::One | Stage::Two => rng.random_bool(1. / (existing_ufo * 10.)),
            Stage::Three | Stage::Four => rng.random_bool(1. / (existing_ufo * 3.)),
            Stage::Five | Stage::Six => rng.random_bool(1. / (existing_ufo)),
        };
    }

    fn get_ufo_velocity(&self) -> Vec2 {
        let mut rng = rng();
        match self {
            Stage::Warmup | Stage::One => Vec2::new(0., -3.),
            Stage::Two | Stage::Three => Vec2::new(rng.random_range(-3.0..3.0), -3.),
            Stage::Four | Stage::Five => {
                Vec2::new(rng.random_range(-5.0..5.0), rng.random_range(-5.0..5.0))
            }
            Stage::Six => Vec2::new(rng.random_range(-10.0..10.0), rng.random_range(-10.0..-5.0)),
        }
    }
}
