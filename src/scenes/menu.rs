use ggez::{graphics, Context};
use ggez::nalgebra::{Point2, Vector2};
use ggez::graphics::{DrawParam};
use ggez::event::{KeyCode, KeyMods};

use super::Assets;
use super::{Scene, SceneSwitch};

pub struct MainMenuScene {}

impl MainMenuScene {
    pub fn new() -> MainMenuScene {
        MainMenuScene {}
    }
}

impl Scene for MainMenuScene {
    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) -> SceneSwitch {
        match keycode {
            KeyCode::Space => {
                SceneSwitch::push(super::PlayingScene::new())
            },
            KeyCode::Escape => {
                ggez::event::quit(ctx);
                SceneSwitch::None
            },
            _ => SceneSwitch::None,
        }
    }

    fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) {
        let draw_string = format!("ENTER: START\nESCAPE: QUIT");
        let draw_text = graphics::Text::new((draw_string, assets.get_font(), 32.0));
        let draw_params_text = DrawParam::new()
            .dest(Point2::new(300.0, 10.0))
            .color(graphics::WHITE)
            .scale(Vector2::new(0.5, 0.5));

        graphics::draw(ctx, &draw_text, draw_params_text).expect("Drawing failed");
    }
}