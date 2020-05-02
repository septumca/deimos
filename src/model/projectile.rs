use super::Position;
use super::Motion;
use super::collision;

use ggez::graphics;
use ggez::graphics::{Rect, DrawParam};
use ggez::nalgebra::{Point2, Vector2};


pub struct Projectile {
    pub alive: bool,
    position: Position,
    motion: Motion,
}

impl Projectile {
    pub fn new(location: Vector2<f32>, velocity: Vector2<f32>, rotation: f32) -> Projectile {
        Projectile {
            position: Position::new(location, rotation),
            motion: Motion::new(velocity, 0.0),
            alive: true
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position.add_motion(&self.motion.mutliply_by(delta_time));
    }

    pub fn check_bounds(&mut self, bounds: &Rect) {
        if !collision::point_in_rect(bounds, &Point2::from(self.position.location)) {
            self.alive = false;
        }
    }

    pub fn get_draw_params(&self) -> DrawParam {
        DrawParam::from((Point2::from(self.position.location), self.position.rotation, graphics::Color::from_rgb(252, 36, 3)))
    }
}