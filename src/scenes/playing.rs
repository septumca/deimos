
use super::super::model;
use super::{Scene, SceneSwitch};
use super::Assets;

use ggez::{graphics, Context, GameResult};
use ggez::graphics::{DrawParam, DrawMode, Mesh};
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::nalgebra::{Point2, Vector2};
use ggez::timer;


pub struct PlayingScene {
    model: model::Model,
}

impl PlayingScene {
    pub fn new() -> PlayingScene {
        let model = model::Model::new();
        PlayingScene {
            model
        }
    }

    fn is_accepting_input(&self) -> bool {
        self.model.state != model::ModelState::ExecutingTurn
    }

    fn draw(ctx: &mut Context, assets: &Assets, model: &mut model::Model) -> GameResult {
        if model.state != model::ModelState::ExecutingTurn {
            PlayingScene::draw_player_helpers(ctx, assets, &model)?;
        }

        PlayingScene::draw_player(ctx, assets, &model.player)?;
        let draw_string = format!("DEIMOS: {:?}: {:?}", model.state, 9);
        let draw_text = graphics::Text::new((draw_string, assets.get_font(), 32.0));
        let draw_params_text = DrawParam::new()
            .dest(Point2::new(300.0, 10.0))
            .color(graphics::WHITE)
            .scale(Vector2::new(0.5, 0.5));

        graphics::draw(ctx, &draw_text, draw_params_text)?;

        for projectile in &mut model.projectiles {
            graphics::draw(ctx, &assets.projectile_mesh, projectile.get_draw_params())?;
        }

        Ok(())
    }

    fn draw_player_helpers(ctx: &mut Context, assets: &Assets, model: &model::Model) -> GameResult {
        let circle_mesh = Mesh::new_circle(
            ctx,
            DrawMode::stroke(1.0),
            Point2::from(model.player.position.location),
            model.player.max_speed * model.turn_duration,
            1.0,
            graphics::Color::from_rgb(128, 128, 128)
        )?;

        if model.player_target_info.rotation != model.player.position.rotation {
            let rotation_draw_params = DrawParam::from((
                Point2::from(model.player.position.location),
                model.player_target_info.rotation,
                graphics::Color::from_rgb(128, 128, 128)
            ));
            graphics::draw(ctx, &assets.destroyer_mesh, rotation_draw_params)?;
        }

        if model.player_target_info.location != model.player.position.location {
            let line_mesh = Mesh::new_line(
                ctx,
                &[Point2::from(model.player_target_info.location), Point2::from(model.player.position.location)],
                1.0,
                graphics::Color::from_rgb(128, 128, 128)
            )?;
            graphics::draw(ctx, &line_mesh, DrawParam::default())?;
        }

        graphics::draw(ctx, &circle_mesh, DrawParam::default())?;

        Ok(())
    }

    fn draw_player(ctx: &mut Context, assets: &Assets, actor: &model::actor::Actor) -> GameResult {
        let actor_draw_params = actor.get_draw_params();

        graphics::draw(ctx, &assets.destroyer_mesh, actor_draw_params)?;

        Ok(())
    }
}

impl Scene for PlayingScene {
    fn update(&mut self, ctx: &mut Context) -> SceneSwitch {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let delta_time = 1.0 / (DESIRED_FPS as f32);
            self.model.update(delta_time);
        }

        SceneSwitch::None
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) -> SceneSwitch {
        if self.is_accepting_input() {
            match keycode {
                KeyCode::Space => {
                    self.model.start_turn_execution();
                },
                KeyCode::M => {
                    self.model.state = model::ModelState::MovementSelection;
                },
                KeyCode::R => {
                    self.model.state = model::ModelState::RotationSelection;
                },
                _ => (),
            }
        }
        match keycode {
            KeyCode::Return => {
                // self.model.spawn_projectile();
            },
            _ => (),
        }

        SceneSwitch::None
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) -> SceneSwitch {
        match self.model.state {
            model::ModelState::MovementSelection => {
                self.model.player.set_target_position(Vector2::new(x, y), self.model.turn_duration);
                self.model.player_target_info.location = self.model.player.position.location + self.model.player.motion.velocity * self.model.turn_duration;
                ()
            }
            model::ModelState::RotationSelection => {
                self.model.player.set_target_rotation(Vector2::new(x, y), self.model.turn_duration);
                self.model.player_target_info.rotation = self.model.player.position.rotation + self.model.player.motion.rotation_speed * self.model.turn_duration;
                ()
            },
            _ => ()
        };

        SceneSwitch::None
    }

    fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) {
        PlayingScene::draw(ctx, assets, &mut self.model)
            .expect("Drawing playing state has failed");
    }
}
