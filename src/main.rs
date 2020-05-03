mod scenes;
pub mod model;
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
    let config = assets::load_config();

    let cb = ContextBuilder::new(&config.window_title, "zarosysatravakosi")
        .window_mode(conf::WindowMode::default().dimensions(config.window_size.0, config.window_size.1))
        .add_resource_path(resource_dir);
    let (ctx, events_loop) = &mut cb.build()?;

    let assets = Assets::new(ctx, config)?;

    println!("ASSETS CONFIG: {:?}", assets.config);
    let mut scene_mgr = scenes::SceneStack::new(assets);

    event::run(ctx, events_loop, &mut scene_mgr)
}