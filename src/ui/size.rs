use bevy::math::Vec2;

pub const MOBILE_WINDOW_SIZE: Vec2 = Vec2::new(828., 1792.);

pub const BULLET_SIZE: Vec2 = Vec2::new(5., 10.);

pub const SPACESHIP_SIZE: Vec2 = Vec2::new(100., 100.);
pub const UFO_SIZE: Vec2 = Vec2::new(80., 54.);

pub fn get_left_edge(object_width: f32) -> f32 {
    (-MOBILE_WINDOW_SIZE.x + object_width) / 2.
}

pub fn get_right_edge(object_width: f32) -> f32 {
    -get_left_edge(object_width)
}

pub fn get_top_edge(object_height: f32) -> f32 {
    (MOBILE_WINDOW_SIZE.x - object_height) / 2.
}

pub fn get_bottom_edge(object_height: f32) -> f32 {
    -get_top_edge(object_height)
}
