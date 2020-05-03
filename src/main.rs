mod scenes;
mod model;
pub mod assets;

use assets::Assets;

use ggez::{ContextBuilder, GameResult};
use ggez::event;
use ggez::conf;

use std::env;
use std::path;


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

    let assets = Assets::new(ctx)?;
    let mut scene_mgr = scenes::SceneStack::new(assets);

    event::run(ctx, events_loop, &mut scene_mgr)
}