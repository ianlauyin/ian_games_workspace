use bevy::prelude::*;

const INITIAL_HEALTH: u8 = 3;

#[derive(Component)]
pub struct Health(pub u8);

impl Health {
    pub fn new() -> Self {
        Self(INITIAL_HEALTH)
    }

    pub fn reduce(&mut self) {
        self.0 -= 1;
    }
}
