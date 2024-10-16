pub enum ZIndexMap {
    Background,
    Stars,
    SpaceShip,
    UFO,
    Bullet,
}

impl ZIndexMap {
    pub fn value(&self) -> f32 {
        match self {
            ZIndexMap::Background => 0.,
            ZIndexMap::Stars => 1.,
            ZIndexMap::SpaceShip | ZIndexMap::UFO | ZIndexMap::Bullet => 2.,
        }
    }
}
