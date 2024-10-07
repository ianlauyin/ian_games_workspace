pub enum ZIndexMap {
    Background,
    Stars,
    SpaceShip,
    UFO,
}

impl ZIndexMap {
    pub fn value(&self) -> f32 {
        match self {
            ZIndexMap::Background => 0.,
            ZIndexMap::Stars => 1.,
            ZIndexMap::SpaceShip | ZIndexMap::UFO => 2.,
        }
    }
}
