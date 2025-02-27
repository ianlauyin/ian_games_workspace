use bevy_math::Vec2;

pub const MOBILE_WINDOW_SIZE: Vec2 = Vec2::new(540., 960.);
pub const UFO_SIZE: Vec2 = Vec2::new(80., 54.);
pub const SPACESHIP_SIZE: Vec2 = Vec2::new(100., 100.);

pub struct EdgeUtil {
    object_size: Vec2,
}

impl EdgeUtil {
    pub fn ufo() -> Self {
        EdgeUtil::new(UFO_SIZE)
    }
    pub fn spaceship() -> Self {
        EdgeUtil::new(SPACESHIP_SIZE)
    }

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
