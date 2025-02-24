use bevy::prelude::*;
use std::f32::consts::PI;

use crate::constant::MOBILE_WINDOW_SIZE;

pub fn angle_to_radian(angle: f32) -> f32 {
    angle * PI / 180.
}

pub fn cleanup_components<T: Component>(
    mut commands: Commands,
    component_q: Query<Entity, With<T>>,
) {
    for entity in component_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct EdgeUtil {
    object_size: Vec2,
}

impl EdgeUtil {
    pub fn new(object_size: Vec2) -> Self {
        Self { object_size }
    }

    pub fn left_in(&self) -> f32 {
        -MOBILE_WINDOW_SIZE.x / 2. + self.object_size.x / 2.
    }
    pub fn over_left_in(&self, position: f32) -> bool {
        position < self.left_in()
    }

    pub fn right_in(&self) -> f32 {
        MOBILE_WINDOW_SIZE.x / 2. - self.object_size.x / 2.
    }
    pub fn over_right_in(&self, position: f32) -> bool {
        position > self.right_in()
    }

    pub fn top_in(&self) -> f32 {
        MOBILE_WINDOW_SIZE.y / 2. - self.object_size.y / 2.
    }
    pub fn over_top_in(&self, position: f32) -> bool {
        position > self.top_in()
    }
    pub fn top_out(&self) -> f32 {
        MOBILE_WINDOW_SIZE.y / 2. + self.object_size.y / 2.
    }
    pub fn over_top_out(&self, position: f32) -> bool {
        position > self.top_out()
    }

    pub fn bottom_in(&self) -> f32 {
        -MOBILE_WINDOW_SIZE.y / 2. + self.object_size.y / 2.
    }
    pub fn over_bottom_in(&self, position: f32) -> bool {
        position < self.bottom_in()
    }
    pub fn bottom_out(&self) -> f32 {
        -MOBILE_WINDOW_SIZE.y / 2. - self.object_size.y / 2.
    }
    pub fn over_bottom_out(&self, position: f32) -> bool {
        position < self.bottom_out()
    }
}
