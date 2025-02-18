pub use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ImageHandles {
    pub explosion: Handle<Image>,
    pub spaceship: Handle<Image>,
    pub ufo: Handle<Image>,
    pub stars: Handle<Image>,
}
