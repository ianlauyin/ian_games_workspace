use bevy::math::Vec2;

pub const FULL_WINDOW_SIZE: Vec2 = Vec2::new(1024., 768.);

const BULLET_SIZE: Vec2 = Vec2::new(5., 10.);
const BULLET_SIZE_MOBILE: Vec2 = Vec2::new(2.5, 5.);

const SPACESHIP_SIZE: Vec2 = Vec2::new(100., 100.);
const SPACESHIP_SIZE_MOBILE: Vec2 = Vec2::new(50., 50.);
const UFO_SIZE: Vec2 = Vec2::new(80., 54.);
const UFO_SIZE_MOBILE: Vec2 = Vec2::new(40., 27.);

pub fn is_mobile(window_width: f32) -> bool {
    window_width < FULL_WINDOW_SIZE.x
}
pub fn get_left_edge(window_width: f32, object_width: f32) -> f32 {
    if is_mobile(window_width) {
        (-window_width + object_width) / 2.
    } else {
        (-FULL_WINDOW_SIZE.x + object_width) / 2.
    }
}

pub fn get_right_edge(window_width: f32, object_width: f32) -> f32 {
    -get_left_edge(window_width, object_width)
}

pub fn get_top_edge(window_height: f32, object_height: f32) -> f32 {
    if window_height >= FULL_WINDOW_SIZE.x {
        (FULL_WINDOW_SIZE.x - object_height) / 2.
    } else {
        (window_height - object_height) / 2.
    }
}

pub fn get_bottom_edge(window_height: f32, object_height: f32) -> f32 {
    -get_top_edge(window_height, object_height)
}

pub fn get_bullet_size(window_width: f32) -> Vec2 {
    if is_mobile(window_width) {
        BULLET_SIZE_MOBILE
    } else {
        BULLET_SIZE
    }
}

pub fn get_spaceship_size(window_width: f32) -> Vec2 {
    if is_mobile(window_width) {
        SPACESHIP_SIZE_MOBILE
    } else {
        SPACESHIP_SIZE
    }
}
pub fn get_ufo_size(window_width: f32) -> Vec2 {
    if is_mobile(window_width) {
        UFO_SIZE_MOBILE
    } else {
        UFO_SIZE
    }
}
