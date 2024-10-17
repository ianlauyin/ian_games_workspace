use bevy::app::App;
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
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
        let new_translation = Vec3::new(
            origin_translation.x + velocity.x,
            origin_translation.y + velocity.y,
            origin_translation.z,
        );
        transform.translation = new_translation;
    }
}
