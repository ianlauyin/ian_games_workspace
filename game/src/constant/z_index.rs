use bevy::ui::ZIndex as BevyZIndex;

pub enum ZIndex {
    BACKGROUND,
    STARS,
    EXPLOSION,
    SPACESHIP,
    UFO,
    BULLET,
    MAINCONTAINER,
    TEXT,
}

impl ZIndex {
    pub fn z_value(&self) -> f32 {
        match self {
            ZIndex::BACKGROUND => 0.,
            ZIndex::STARS => 1.,
            ZIndex::EXPLOSION => 2.,
            ZIndex::SPACESHIP | ZIndex::UFO | ZIndex::BULLET => 3.,
            ZIndex::MAINCONTAINER => 4.,
            ZIndex::TEXT => 5.,
        }
    }

    pub fn component(&self) -> BevyZIndex {
        BevyZIndex(self.z_value() as i32)
    }
}
