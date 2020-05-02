use ggez::nalgebra::{Vector2};

pub struct Motion {
    pub rotation_speed: f32,
    pub velocity: Vector2<f32>
}

impl Motion {
    pub fn new(velocity: Vector2<f32>, rotation_speed: f32) -> Motion {
        Motion {
            rotation_speed,
            velocity
        }
    }

    pub fn zero() -> Motion {
        Motion {
            rotation_speed: 0.0,
            velocity: Vector2::new(0.0, 0.0)
        }
    }

    pub fn mutliply_by(&self, factor: f32) -> Motion {
        Motion {
            rotation_speed: self.rotation_speed * factor,
            velocity: self.velocity * factor
        }
    }
}
