#[path = "./collision.rs"] mod collision;

use ggez::{graphics};
use ggez::nalgebra::{Point2, Vector2};


pub struct Projectile {
    pub alive: bool,
    velocity: Vector2<f32>,
    current_position: Vector2<f32>,
}

impl Projectile {
    pub fn new(position: Vector2<f32>, velocity: Vector2<f32>) -> Projectile {
        // let velocity = super::normalize_vector_by_speed(position, target_position, speed);

        Projectile {
            current_position: position,
            velocity,
            alive: true
        }
    }

    pub fn update(&mut self, delta: f32) {
        let velocity_per_time_unit = self.velocity * delta;
        self.current_position = self.current_position + velocity_per_time_unit;
    }

    pub fn check_bounds(&mut self, bounds: &graphics::Rect) {
        if !collision::point_in_rect(bounds, &Point2::from(self.current_position)) {
            println!("REMOVING PROJECTILE");
            self.alive = false;
        }
    }

    pub fn get_draw_params(&self) -> graphics::DrawParam {
        graphics::DrawParam::from((Point2::from(self.current_position), 0.0, graphics::Color::from_rgb(252, 36, 3)))
    }
}