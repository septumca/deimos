use ggez::{graphics};
use ggez::nalgebra::{Point2, Vector2, Matrix, Rotation2};
use std::fmt;

use super::Motion;
use super::Position;
use super::Cooldown;

pub struct Gun {
    cooldown: Cooldown
}

impl Gun {
    pub fn new(cooldown_time: f32) -> Gun {
        Gun {
            cooldown: Cooldown::new(cooldown_time)
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.cooldown.update(delta_time);
    }

    pub fn can_shoot(&self) -> bool {
        self.cooldown.has_elapsed()
    }
}

pub struct Actor {
    pub max_speed: f32,
    pub max_rotation_speed: f32,

    pub position: Position,
    pub motion: Motion,

    pub gun_port: Gun,
    pub gun_starboard: Gun,
}

impl Actor {
    pub fn new() -> Actor {
        let position = Vector2::new(200.0, 200.0);
        let gun_port = Gun::new(1.5);
        let gun_starboard = Gun::new(1.5);
        Actor {
            max_speed: 20.0,
            max_rotation_speed: 0.2,
            position: Position::new(position, 0.0),
            motion: Motion::zero(),
            gun_port,
            gun_starboard
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position.add_motion(&self.motion.mutliply_by(delta_time));

        self.gun_port.update(delta_time);
        self.gun_starboard.update(delta_time);
    }

    pub fn reset_movement(&mut self) {
        self.motion = Motion::zero();
    }

    pub fn get_draw_params(&self) -> graphics::DrawParam {
        graphics::DrawParam::from((Point2::from(self.position.location), self.position.rotation, graphics::WHITE))
    }

    pub fn set_target_position(&mut self, target_position: Vector2<f32>, turn_time: f32) {
        let distance = target_position - self.position.location;
        let max_scalar_distance_in_turn_time = self.max_speed * turn_time;
        let velocity_norm_sq = distance.norm_squared();

        self.motion.velocity = if velocity_norm_sq > max_scalar_distance_in_turn_time.powi(2) {
            distance / velocity_norm_sq.sqrt() * self.max_speed
        } else {
            distance / turn_time
        };
    }

    pub fn set_target_rotation(&mut self, rotate_to: Vector2<f32>, turn_time: f32) {
        let rotation_vector = super::vec_from_angle(self.position.rotation);
        let rotate_to_normalized: Vector2<f32> =
            (rotate_to - self.position.location)
                .try_normalize(0.0)
                .unwrap_or(Vector2::new(rotation_vector.x, rotation_vector.y));

        let rotation = Matrix::angle(&rotation_vector, &rotate_to_normalized);
        let cross_product = rotation_vector.x * rotate_to_normalized.y - rotation_vector.y * rotate_to_normalized.x;

        self.motion.rotation_speed = if rotation > self.max_rotation_speed * turn_time {
            self.max_rotation_speed * cross_product.signum()
        } else {
            rotation * cross_product.signum() / turn_time
        };
    }

    pub fn fire_startboard_gun(&mut self) -> Option<super::projectile::Projectile> {
        if !self.gun_starboard.can_shoot() {
            return Option::None;
        }

        let offset = Vector2::new(20.0, 10.0);
        let rotation = Rotation2::new(self.position.rotation);
        let projectile_speed = 100.0;

        let projectile_velocity = super::vec_from_angle(self.position.rotation + (std::f32::consts::PI / 2.0)) * projectile_speed;
        let projectile_position = self.position.location + (rotation * offset);

        self.gun_starboard.cooldown.reset();

        Some(super::projectile::Projectile::new(projectile_position, projectile_velocity, self.position.rotation))
    }
}

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Actor:")
    }
}

// impl super::Movable for DActor {
//     fn move_to(&mut self, position) {

//     }
// }