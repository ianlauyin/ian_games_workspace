pub struct PlayerInfo {
    score: u16,
    health: u16,
    position: (f32, f32),
    bullets: Vec<(f32, f32)>,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            score: 0,
            health: 3,
            position: (0.0, 0.0),
            bullets: Vec::new(),
        }
    }
}
