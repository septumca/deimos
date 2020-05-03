use ggez::graphics::{Rect};
use ggez::nalgebra::{Point2};

use super::super::assets::{ShipConfig, Config};


pub struct Ship {
    pub max_speed: f32,
    pub max_rotation_speed: f32,
    pub frame_rect: Rect,
    pub offset: Point2<f32>
}

impl Ship {
    pub fn from_config(ship_config: &ShipConfig, config: &Config) -> Ship {
        let frame_rect = Rect::new(
            ship_config.frame_rect.0 / config.image_source_dimensions.0,
            ship_config.frame_rect.1 / config.image_source_dimensions.1,
            ship_config.frame_rect.2 / config.image_source_dimensions.0,
            ship_config.frame_rect.3 / config.image_source_dimensions.1,
        );

        Ship {
            max_speed: ship_config.max_speed,
            max_rotation_speed: ship_config.max_rotation_speed,
            frame_rect,
            offset: Point2::new(ship_config.offset.0, ship_config.offset.1),
        }
    }
}