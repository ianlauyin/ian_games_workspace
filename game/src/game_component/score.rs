use bevy::prelude::*;

#[derive(Component)]
pub struct Score(pub u8);

impl Score {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add(&mut self, amount: u8) {
        self.0 += amount;
    }
}
