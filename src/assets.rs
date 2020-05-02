use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Rect, DrawMode, Mesh};
use ggez::nalgebra::{Point2};


pub struct Assets {
    image: graphics::Image,
    font: graphics::Font,
    pub fighter_mesh: graphics::Mesh,
    pub fighter_points: Vec<Point2<f32>>,
    pub bomber_mesh: graphics::Mesh,
    pub bomber_points: Vec<Point2<f32>>,
    pub destroyer_mesh: graphics::Mesh,
    pub destroyer_points: Vec<Point2<f32>>,
    pub projectile_mesh: graphics::Mesh,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut image = graphics::Image::new(ctx, "/deimos-ships.png")?;
        let font = graphics::Font::new(ctx, "/PixelFJVerdana12pt.ttf")?;
        image.set_filter(graphics::FilterMode::Nearest);

        let destroyer_points: Vec<Point2<f32>> =  vec![
            Point2::new(25.0, -7.0),
            Point2::new(25.0, 7.0),
            Point2::new(-5.0, 15.0),
            Point2::new(-20.0, 12.0),
            Point2::new(-20.0, 12.0),
            Point2::new(-20.0, -12.0),
            Point2::new(-5.0, -15.0),
        ];

        let fighter_points: Vec<Point2<f32>> =  vec![
            Point2::new(5.0, 0.0),
            Point2::new(-5.0, 3.0),
            Point2::new(5.0, -3.0),
        ];

        let bomber_points: Vec<Point2<f32>> =  vec![
            Point2::new(5.0, 0.0),
            Point2::new(-5.0, 3.0),
            Point2::new(5.0, -3.0),
        ];

        let destroyer_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::stroke(1.0),
            &destroyer_points,
            graphics::WHITE
        )?;

        let fighter_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::stroke(1.0),
            &fighter_points,
            graphics::WHITE
        )?;

        let bomber_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::stroke(1.0),
            &bomber_points,
            graphics::WHITE
        )?;

        let projectile_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            Rect::new(-1.0, -1.0, 2.0, 2.0),
            graphics::WHITE)?;

        Ok(Assets {
            image,
            font,
            destroyer_mesh,
            destroyer_points: destroyer_points,
            fighter_mesh,
            fighter_points: fighter_points,
            bomber_mesh,
            bomber_points: bomber_points,
            projectile_mesh
        })
    }

    pub fn get_font(&self) -> graphics::Font {
        self.font
    }

    pub fn get_image(&self) -> &graphics::Image {
        &self.image
    }
}