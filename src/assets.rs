use ggez::{graphics, Context, GameResult};

use ron::de::from_reader;
use serde::Deserialize;
use std::{fs::File};


#[derive(Deserialize, Debug)]
pub struct Config {
    pub window_size: (f32, f32),
    pub window_title: String,
    pub fullscreen: bool,

    pub font_source_name: String,
    pub image_source_name: String,
    pub image_source_dimensions: (f32,f32),
    pub ships: ShipsCollectionConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShipConfig {
    pub collision_shape_points: Vec<(f32, f32)>,
    pub max_speed: f32,
    pub max_rotation_speed: f32,
    pub frame_rect: (f32, f32, f32, f32),
    pub offset: (f32, f32),
}

#[derive(Deserialize, Debug)]
pub struct ShipsCollectionConfig {
    pub destroyer: ShipConfig,
    pub fighter: ShipConfig
}


pub struct Assets {
    pub image: graphics::Image,
    pub font: graphics::Font,
    pub config: Config,
}

impl Assets {
    pub fn new(ctx: &mut Context, config: Config) -> GameResult<Assets> {
        let mut image = graphics::Image::new(ctx, format!("/{}", &config.image_source_name))?;
        let font = graphics::Font::new(ctx, format!("/{}", &config.font_source_name))?;
        image.set_filter(graphics::FilterMode::Nearest);

        // let destroyer_points: Vec<Point2<f32>> =  vec![
        //     Point2::new(25.0, -7.0),
        //     Point2::new(25.0, 7.0),
        //     Point2::new(-5.0, 15.0),
        //     Point2::new(-20.0, 12.0),
        //     Point2::new(-20.0, 12.0),
        //     Point2::new(-20.0, -12.0),
        //     Point2::new(-5.0, -15.0),
        // ];

        // let fighter_points: Vec<Point2<f32>> =  vec![
        //     Point2::new(5.0, 0.0),
        //     Point2::new(-5.0, 3.0),
        //     Point2::new(5.0, -3.0),
        // ];

        // let bomber_points: Vec<Point2<f32>> =  vec![
        //     Point2::new(5.0, 0.0),
        //     Point2::new(-5.0, 3.0),
        //     Point2::new(5.0, -3.0),
        // ];

        // let destroyer_mesh = Mesh::new_polygon(
        //     ctx,
        //     DrawMode::stroke(1.0),
        //     &destroyer_points,
        //     graphics::WHITE
        // )?;

        // let fighter_mesh = Mesh::new_polygon(
        //     ctx,
        //     DrawMode::stroke(1.0),
        //     &fighter_points,
        //     graphics::WHITE
        // )?;

        // let bomber_mesh = Mesh::new_polygon(
        //     ctx,
        //     DrawMode::stroke(1.0),
        //     &bomber_points,
        //     graphics::WHITE
        // )?;

        // let projectile_mesh = Mesh::new_rectangle(
        //     ctx,
        //     DrawMode::stroke(1.0),
        //     Rect::new(-1.0, -1.0, 2.0, 2.0),
        //     graphics::WHITE)?;

        Ok(Assets {
            image,
            font,
            config
        })
    }
}

pub fn load_config() -> Config {
    let input_path = format!("{}/resources/game-config.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(input_path).expect("Failed opening config file");
    from_reader(f).expect("Filed parsing config file")
}