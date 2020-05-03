pub mod actor;
pub mod projectile;
pub mod components;
pub mod ship;

use components::{Position, Motion, Cooldown, collision};
use super::Assets;

use ggez::graphics::{Rect};
use ggez::nalgebra::{Vector2};
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum ModelState {
    WaitingForInput,
    MovementSelection,
    RotationSelection,
    ExecutingTurn
}

pub struct Model {
    pub player: actor::Actor,
    pub player_target_info: Position,
    pub projectiles: Vec<projectile::Projectile>,
    pub state: ModelState,
    animation_update_cycle: i8,
    current_cycle: i8,
    pub turn_duration: f32,
    current_turn_duration: f32,
    bounds: Rect,
}

impl Model {
    pub fn new(assets: &Assets) -> Model {
        let player = actor::Actor::new(assets);
        let player_target_info = Position::new(Vector2::new(player.position.location.x, player.position.location.y), player.position.rotation);

        Model {
            player,
            player_target_info,
            projectiles: vec![],
            animation_update_cycle: 10,
            current_cycle: 0,
            state: ModelState::WaitingForInput,
            turn_duration: 5.0,
            current_turn_duration: 0.0,
            bounds: Rect::new(0.0, 0.0, 5000.0, 5000.0),
        }
    }

    fn should_update_turn(&self) -> bool {
        self.state == ModelState::ExecutingTurn
    }

    fn update_turn(&mut self, delta_time: f32) {
        self.player.update(delta_time);

        let wrapped_projectile = self.player.fire_startboard_gun();

        match wrapped_projectile {
            Some(projectile) => {
                self.projectiles.push(projectile);
            },
            _ => ()
        };

        for projectile in &mut self.projectiles {
            projectile.update(delta_time);
            projectile.check_bounds(&self.bounds)
        }
        self.remove_dead_projectiles();

        self.current_turn_duration = self.current_turn_duration + delta_time;

        if self.current_turn_duration > self.turn_duration {
            self.reset_after_execution_end();
        }
    }

    fn reset_after_execution_end(&mut self) {
        self.state = ModelState::WaitingForInput;
        self.current_turn_duration = 0.0;
        self.player.reset_movement();
        self.player_target_info = Position::new(
            Vector2::new(self.player.position.location.x, self.player.position.location.y),
            self.player.position.rotation
        );
    }

    fn remove_dead_projectiles(&mut self) {
        self.projectiles.retain(|projectile| projectile.alive);
    }

    fn update_animations(&mut self) {
        if self.current_cycle == self.animation_update_cycle {
            self.current_cycle = 0;
        }
    }

    pub fn start_turn_execution(&mut self) {
        self.state = ModelState::ExecutingTurn;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_cycle = self.current_cycle + 1;
        if self.should_update_turn() {
            self.update_turn(delta_time);
        }

        self.update_animations();
    }
}




impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Model:
    projectiles: {},
    animation_update_cycle: {},
    current_cycle: {},
    turn_duration: {},
    current_turn_duration: {},
    state: {:?}", self.projectiles.len(), self.animation_update_cycle, self.current_cycle, self.turn_duration, self.current_turn_duration, self.state)
    }
}

pub trait WithFrame {
    fn get_frame(&self) -> &Rect;
}

// fn points_to_frames(frames: Vec<(f32, f32)>) -> Vec<graphics::Rect> {
//     frames
//         .iter()
//         .map(|(x, y)| {
//             graphics::Rect::new(
//                 constants::FRAME_WIDTH_RATIO * x,
//                 constants::FRAME_HEIGHT_RATIO * y,
//                 constants::FRAME_WIDTH_RATIO,
//                 constants::FRAME_HEIGHT_RATIO
//             )
//         }

//         )
//         .collect()
// }

fn vec_from_angle(angle: f32) -> Vector2<f32> {
    let vx = angle.cos();
    let vy = angle.sin();
    Vector2::new(vx, vy)
}