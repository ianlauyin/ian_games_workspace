use bevy::math::Vec2;

pub const WINDOW_SIZE: Vec2 = Vec2::new(1024., 768.);
pub const LEFT_EDGE: f32 = -WINDOW_SIZE.x / 2. + 50.;
pub const RIGHT_EDGE: f32 = WINDOW_SIZE.x / 2. - 50.;
pub const TOP_EDGE: f32 = WINDOW_SIZE.y / 2. - 50.;
pub const BOTTOM_EDGE: f32 = -WINDOW_SIZE.y / 2. + 50.;

pub const BULLET_SIZE: Vec2 = Vec2::new(5., 10.);

pub const SPACESHIP_SIZE: Vec2 = Vec2::new(100., 100.);
pub const UFO_SIZE: Vec2 = Vec2::new(80., 54.);
