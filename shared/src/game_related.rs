use bevy_math::Vec2;
use rand::{rng, Rng};

use crate::util::EdgeUtil;

#[derive(Default, Clone)]
pub enum Stage {
    #[default]
    Warmup,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Stage {
    pub fn new(score: u8) -> Stage {
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

    pub fn random_generator(&self, existing_ufo: usize) -> bool {
        let mut rng = rng();
        return match self {
            Stage::Warmup => rng.random_bool(0.01),
            Stage::One | Stage::Two => rng.random_bool(1. / (existing_ufo as f64 * 10.)),
            Stage::Three | Stage::Four => rng.random_bool(1. / (existing_ufo as f64 * 3.)),
            Stage::Five | Stage::Six => rng.random_bool(1. / (existing_ufo as f64)),
        };
    }

    pub fn get_ufo_velocity(&self) -> Vec2 {
        let mut rng = rng();
        match self {
            Stage::Warmup | Stage::One => Vec2::new(0., -3.),
            Stage::Two | Stage::Three => Vec2::new(rng.random_range(-3.0..3.0), -3.),
            Stage::Four | Stage::Five => {
                Vec2::new(rng.random_range(-5.0..5.0), rng.random_range(-10.0..-5.0))
            }
            Stage::Six => Vec2::new(rng.random_range(-10.0..10.0), rng.random_range(-10.0..-5.0)),
        }
    }

    pub fn get_ufo_velocity_tuple(&self) -> (f32, f32) {
        let velocity = self.get_ufo_velocity();
        (velocity.x, velocity.y)
    }
}

pub struct UFORandomGenerator;

impl UFORandomGenerator {
    pub fn tag() -> u128 {
        let mut rng = rng();
        rng.random_range(u128::MIN..u128::MAX)
    }

    pub fn position() -> (f32, f32) {
        let ufo_edge = EdgeUtil::ufo();
        let mut rng = rng();
        (
            rng.random_range(ufo_edge.left_in()..ufo_edge.right_in()),
            ufo_edge.bottom_out(),
        )
    }
}
