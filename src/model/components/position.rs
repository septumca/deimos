use ggez::nalgebra::{Vector2};
use super::motion::Motion;

pub struct Position {
    pub rotation: f32,
    pub location: Vector2<f32>
}

impl Position {
    pub fn new(location: Vector2<f32>, rotation: f32) -> Position {
        Position {
            rotation,
            location
        }
    }

    pub fn add_motion(&mut self, motion: &Motion) {
        self.location = self.location + motion.velocity;
        self.rotation = self.rotation + motion.rotation_speed;
    }
}
