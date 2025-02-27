use std::mem::swap;

use bevy::prelude::*;

#[derive(Component)]
pub struct Blink {
    appearing: bool,
    speed: f32,
    current_alpha: f32,
    max_alpha: f32,
    min_alpha: f32,
}

impl Default for Blink {
    fn default() -> Self {
        Self {
            appearing: true,
            speed: 1.,
            current_alpha: 1.,
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
            current_alpha: min_alpha,
            max_alpha,
            min_alpha,
            ..default()
        }
    }

    pub fn new_with_speed(speed: f32) -> Self {
        Self { speed, ..default() }
    }

    fn get_alpha(&mut self) -> f32 {
        self.current_alpha
    }

    fn update_alpha(&mut self) {
        self.current_alpha += if self.appearing {
            self.speed
        } else {
            -self.speed
        };
        self.check_alpha();
    }

    fn check_alpha(&mut self) {
        if self.current_alpha >= self.max_alpha {
            self.appearing = false;
            return;
        }
        if self.current_alpha <= self.min_alpha {
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

fn handle_blink(mut blink_query: Query<(&mut Blink, Option<&mut Sprite>, Option<&mut TextColor>)>) {
    for (mut blink, sprite_op, text_color_op) in blink_query.iter_mut() {
        blink.update_alpha();
        let new_alpha = blink.get_alpha();
        alter_alpha(sprite_op, text_color_op, new_alpha)
    }
}

fn alter_alpha(sprite_op: Option<Mut<Sprite>>, text_color_op: Option<Mut<TextColor>>, alpha: f32) {
    if let Some(mut sprite) = sprite_op {
        sprite.color.set_alpha(alpha);
    }
    if let Some(mut text_color) = text_color_op {
        text_color.set_alpha(alpha);
    }
}
