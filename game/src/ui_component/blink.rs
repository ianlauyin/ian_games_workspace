use std::mem::swap;

use bevy::prelude::*;

#[derive(Component)]
#[require(Sprite)]
pub struct Blink {
    appearing: bool,
    speed: f32,
    max_alpha: f32,
    min_alpha: f32,
}

impl Default for Blink {
    fn default() -> Self {
        Self {
            appearing: true,
            speed: 1.,
            max_alpha: 1.,
            min_alpha: 0.,
        }
    }
}

impl Blink {
    pub fn new(speed: f32, mut max_alpha: f32, mut min_alpha: f32) -> Self {
        if max_alpha < min_alpha {
            warn!("max_alpha must be greater than min_alpha");
            swap(&mut max_alpha, &mut min_alpha);
        };
        Self {
            speed,
            max_alpha,
            min_alpha,
            ..default()
        }
    }

    pub fn new_with_range(mut max_alpha: f32, mut min_alpha: f32) -> Self {
        if max_alpha < min_alpha {
            warn!("max_alpha must be greater than min_alpha");
            swap(&mut max_alpha, &mut min_alpha);
        };
        Self {
            max_alpha,
            min_alpha,
            ..default()
        }
    }

    pub fn new_with_speed(speed: f32) -> Self {
        Self { speed, ..default() }
    }

    fn check_alpha(&mut self, current_alpha: f32) {
        if current_alpha >= self.max_alpha {
            self.appearing = false;
            return;
        }
        if current_alpha <= self.min_alpha {
            self.appearing = true;
        }
    }
}

pub struct BlinkPlugin;

impl Plugin for BlinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_blink);
    }
}

fn handle_blink(mut blink_query: Query<(&mut Blink, &mut Sprite)>) {
    for (mut blink, mut sprite) in blink_query.iter_mut() {
        let alpha = sprite.color.alpha();
        blink.check_alpha(alpha);
        let new_alpha = if blink.appearing {
            alpha + blink.speed
        } else {
            alpha - blink.speed
        };
        sprite.color.set_alpha(new_alpha)
    }
}
