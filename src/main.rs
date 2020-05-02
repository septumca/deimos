mod assets;
mod model;

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{DrawParam, DrawMode, Mesh};
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::nalgebra::{Point2, Vector2};
use ggez::conf;
use ggez::timer;

use std::env;
use std::path;


struct MainState<'a> {
    assets: &'a mut assets::Assets,
    model: model::Model,
}

impl<'a> MainState<'a> {
    pub fn new(assets: &'a mut assets::Assets) -> MainState<'a> {
        let model = model::Model::new();
        MainState {
            assets,
            model
        }
    }

    fn is_accepting_input(&self) -> bool {
        self.model.state != model::ModelState::ExecutingTurn
    }
}

impl<'a> EventHandler for MainState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let delta_time = 1.0 / (DESIRED_FPS as f32);
            self.model.update(delta_time);
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
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
                // KeyCode::F => {
                //     self.state = StateType::FiringSelection;
                // },
                _ => (),
            }
        }
        match keycode {
            KeyCode::Return => {
                // self.model.spawn_projectile();
            },
            _ => (),
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
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
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if self.model.state != model::ModelState::ExecutingTurn {
            draw_player_helpers(ctx, &self.assets, &self.model)?;
        }

        draw_player(ctx, &self.assets, &self.model.player)?;
        let draw_string = format!("DEIMOS: {:?}: {:?}", self.model.state, 9);
        let draw_text = graphics::Text::new((draw_string, self.assets.get_font(), 32.0));
        let draw_params_text = DrawParam::new()
            .dest(Point2::new(300.0, 10.0))
            .color(graphics::WHITE)
            .scale(Vector2::new(0.5, 0.5));

        graphics::draw(ctx, &draw_text, draw_params_text)?;

        for projectile in &mut self.model.projectiles {
            graphics::draw(ctx, &self.assets.projectile_mesh, projectile.get_draw_params())?;
        }

        timer::yield_now();
        graphics::present(ctx)
    }
}

fn draw_player_helpers(ctx: &mut Context, assets: &assets::Assets, model: &model::Model) -> GameResult {
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

fn draw_player(ctx: &mut Context, assets: &assets::Assets, actor: &model::actor::Actor) -> GameResult {
    let actor_draw_params = actor.get_draw_params();

    graphics::draw(ctx, &assets.destroyer_mesh, actor_draw_params)?;

    Ok(())
}


fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("deimos", "ggez")
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir);
    let (ctx, events_loop) = &mut cb.build()?;

    let mut assets = assets::Assets::new(ctx)?;
    let mut game = MainState::new(&mut assets);

    event::run(ctx, events_loop, &mut game)
}