pub trait Delta {
    fn set_delta(&mut self, delta: f32);
}

#[derive(Default)]
pub struct IdleDelta(pub f32);

impl Delta for IdleDelta {
    fn set_delta(&mut self, delta: f32) {
        self.0 = delta;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameOver {
    Win,
    Lose,
}

#[derive(Default)]
pub struct PhysicsDelta(pub f32);

impl Delta for PhysicsDelta {
    fn set_delta(&mut self, delta: f32) {
        self.0 = delta;
    }
}
