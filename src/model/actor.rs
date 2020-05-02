use ggez::{graphics};
use ggez::nalgebra::{Point2, Vector2, Matrix, Rotation2};
use std::fmt;

pub struct Gun {
    current_cooldown: f32,
    cooldown: f32
}

impl Gun {
    pub fn new(cooldown: f32) -> Gun {
        Gun {
            cooldown,
            current_cooldown: 0.0
        }
    }

    pub fn update_cooldown(&mut self, delta_time: f32) {
        if self.current_cooldown > 0.0 {
            self.current_cooldown = self.current_cooldown - delta_time;
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.current_cooldown <= 0.0
    }

    pub fn refresh_cooldown(&mut self) {
        self.current_cooldown = self.cooldown;
    }
}

pub struct Actor {
    pub max_speed: f32,
    pub velocity: Vector2<f32>,

    pub max_rotation_speed: f32,
    pub rotation_speed: f32,

    pub current_position: Vector2<f32>,
    pub rotation: f32,

    pub target_position: Option<Vector2<f32>>,
    pub target_rotation: f32,

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
            velocity: Vector2::new(0.0, 0.0),
            max_rotation_speed: 0.2,
            rotation_speed: 0.0,
            rotation: 0.0,
            current_position: position,
            target_position: Option::None,
            target_rotation: 0.0,
            gun_port,
            gun_starboard
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_position = self.current_position + (self.velocity * delta_time);
        self.rotation = self.rotation + self.rotation_speed * delta_time;

        self.gun_port.update_cooldown(delta_time);
        self.gun_starboard.update_cooldown(delta_time);
    }

    pub fn reset_movement(&mut self) {
        self.velocity = Vector2::new(0.0, 0.0);
        self.rotation_speed = 0.0;
        self.target_rotation = 0.0;
        self.target_position = Option::None;
    }

    pub fn get_draw_params(&self) -> graphics::DrawParam {
        graphics::DrawParam::from((Point2::from(self.current_position), self.rotation, graphics::WHITE))
    }

    pub fn set_target_position(&mut self, target_position: Vector2<f32>, turn_time: f32) {
        let velocity = target_position - self.current_position;
        let max_scalar_distance_in_turn_time = self.max_speed * turn_time;
        let velocity_norm_sq = velocity.norm_squared();

        if velocity_norm_sq > max_scalar_distance_in_turn_time.powi(2) {
            self.velocity = velocity / velocity_norm_sq.sqrt() * self.max_speed;
        } else {
            self.velocity = velocity / turn_time;
        }

        self.target_position = Some(self.current_position + (self.velocity * turn_time));
    }

    pub fn set_target_rotation(&mut self, rotate_to: Vector2<f32>, turn_time: f32) {
        let rotation_vector = super::vec_from_angle(self.rotation);
        let rotate_to_normalized: Vector2<f32> =
            (rotate_to - self.current_position)
                .try_normalize(0.0)
                .unwrap_or(Vector2::new(rotation_vector.x, rotation_vector.y));

        let rotation = Matrix::angle(&rotation_vector, &rotate_to_normalized);
        let cross_product = rotation_vector.x * rotate_to_normalized.y - rotation_vector.y * rotate_to_normalized.x;

        if rotation > self.max_rotation_speed * turn_time {
            self.rotation_speed = self.max_rotation_speed * cross_product.signum();
        } else {
            self.rotation_speed = rotation * cross_product.signum() / turn_time;
        }

        self.target_rotation = self.rotation + (self.rotation_speed * turn_time);
    }

    pub fn fire_startboard_gun(&mut self) -> Option<super::projectile::Projectile> {
        if !self.gun_starboard.can_shoot() {
            return Option::None;
        }

        let offset = Vector2::new(20.0, 10.0);
        let rotation = Rotation2::new(self.rotation);
        let projectile_speed = 100.0;

        let projectile_velocity = super::vec_from_angle(self.rotation + (std::f32::consts::PI / 2.0)) * projectile_speed;
        let projectile_position = self.current_position + (rotation * offset);

        self.gun_starboard.refresh_cooldown();

        Some(super::projectile::Projectile::new(projectile_position, projectile_velocity))
    }
}

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target_position_str = match self.target_position {
            Some(tp) => format!("{}", tp),
            None => String::from("None")
        };

        write!(f, "Actor:")
    }
}

// impl super::Movable for DActor {
//     fn move_to(&mut self, position) {

//     }
// }