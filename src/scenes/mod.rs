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
    fn update(&mut self, _ctx: &mut Context, _assets: &Assets) -> SceneSwitch {
        SceneSwitch::None
    }
    fn key_up_event(&mut self, _ctx: &mut Context, _assets: &Assets, _keycode: KeyCode, _keymods: KeyMods) -> SceneSwitch {
        SceneSwitch::None
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _assets: &Assets, _button: MouseButton, _x: f32, _y: f32) -> SceneSwitch {
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
        let current_scene = &mut **self.scenes.last_mut() //WTF? - why it cannot be as a function call?
            .expect("No available states: update - critical error in Scenes implementation");
        let next_scene = current_scene.update(ctx, &self.assets);
        self.switch(next_scene);

        Ok(())
    }
    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        let current_scene = &mut **self.scenes.last_mut() //WTF? - why it cannot be as a function call?
            .expect("No available states: key_up_event - critical error in Scenes implementation");
        let next_scene = current_scene.key_up_event(ctx, &self.assets, keycode, keymods);
        self.switch(next_scene);
    }
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let current_scene = &mut **self.scenes.last_mut() //WTF? - why it cannot be as a function call?
            .expect("No available states: mouse_button_up_event - critical error in Scenes implementation");
        let next_scene = current_scene.mouse_button_up_event(ctx, &self.assets, button, x, y);
        self.switch(next_scene);
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let current_scene = &mut **self.scenes.last_mut() //WTF? - why it cannot be as a function call?
            .expect("No available states: draw - critical error in Scenes implementation");
        current_scene.draw(ctx, &mut self.assets);

        timer::yield_now();
        graphics::present(ctx)
    }
}
