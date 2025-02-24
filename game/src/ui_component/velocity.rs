use bevy::app::App;
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn from_vec2(vec2: Vec2) -> Self {
        Self {
            x: vec2.x,
            y: vec2.y,
        }
    }
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_velocity);
    }
}

fn apply_velocity(mut items: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in items.iter_mut() {
        let origin_translation = transform.translation;
        transform.translation.x = origin_translation.x + velocity.x;
        transform.translation.y = origin_translation.y + velocity.y;
    }
}
