mod menu;
mod playing;

use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::timer;

use playing::PlayingScene;
use menu::MainMenuScene;
use super::assets::Assets;

pub enum SceneSwitch {
    None,
    Push(Box<dyn Scene>)
}

impl SceneSwitch {
    pub fn push<S>(scene: S) -> Self
    where
        S: Scene + 'static,
    {
        SceneSwitch::Push(Box::new(scene))
    }
}

pub trait Scene {
    fn update(&mut self, _ctx: &mut Context) -> SceneSwitch {
        SceneSwitch::None
    }
    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) -> SceneSwitch {
        SceneSwitch::None
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> SceneSwitch {
        SceneSwitch::None
    }
    fn draw(&mut self, ctx: &mut Context, assets: &mut Assets);
}

pub struct SceneStack {
    pub assets: Assets,
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneStack {
    pub fn new(assets: Assets) -> SceneStack {
        SceneStack {
            assets,
            scenes: vec![Box::new(MainMenuScene::new())]
        }
    }

    pub fn get_current_scene(&mut self) -> &mut dyn Scene {
        &mut **self.scenes.last_mut() //WTF?
            .expect("No available states - critical error in Scenes implementation")
    }

    fn switch(&mut self, next_scene: SceneSwitch) {
        match next_scene {
            SceneSwitch::None => (),
            SceneSwitch::Push(s) => {
                self.scenes.push(s);
                ()
            }
        }
    }
}

impl EventHandler for SceneStack {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let current_scene = self.get_current_scene();
        let next_scene = current_scene.update(ctx);
        self.switch(next_scene);

        Ok(())
    }
    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        let current_scene = self.get_current_scene();
        let next_scene = current_scene.key_up_event(ctx, keycode, keymods);
        self.switch(next_scene);
    }
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let current_scene = self.get_current_scene();
        let next_scene = current_scene.mouse_button_up_event(ctx, button, x, y);
        self.switch(next_scene);
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        //WHY get_current_scene IS NOT WORKING?
        if let Some((current, _)) = self.scenes.split_last_mut() {
            current.draw(ctx, &mut self.assets);
        }

        timer::yield_now();
        graphics::present(ctx)
    }
}
