use ggez::nalgebra::{Point2};
use ggez::graphics::{Rect};

use super::Position;

const INF: f32 = 1000000.0;

pub struct PolygonShape {
    points: Vec<Point2<f32>>
}

impl PolygonShape {
    pub fn new(points: Vec<Point2<f32>>) -> PolygonShape {
        PolygonShape {
            points: points.clone()
        }
    }
}

impl Collidable for PolygonShape {
    fn update(&mut self, position: &Position) {

    }

    fn collide_with_point(&self, point: &Point2<f32>) -> bool {
        false
    }
}

trait Collidable {
    fn update(&mut self, position: &Position);
    fn collide_with_point(&self, point: &Point2<f32>) -> bool;
}


pub fn line_intersection(line1_start: &Point2<f32>, line1_end: &Point2<f32>, line2_start: &Point2<f32>, line2_end: &Point2<f32>) -> bool {
    // float uA = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
    let u_a: f32 = (
        (line2_end.x-line2_start.x)*(line1_start.y-line2_start.y) -
        (line2_end.y-line2_start.y)*(line1_start.x-line2_start.x)
    ) /
    (
        (line2_end.y-line2_start.y)*(line1_end.x-line1_start.x) -
        (line2_end.x-line2_start.x)*(line1_end.y-line1_start.y)
    );

    // float uB = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
    let u_b: f32 =
    (
        (line1_end.x-line1_start.x)*(line1_start.y-line2_start.y) -
        (line1_end.y-line1_start.y)*(line1_start.x-line2_start.x)
    ) /
    (
        (line2_end.y-line2_start.y)*(line1_end.x-line1_start.x) -
        (line2_end.x-line2_start.x)*(line1_end.y-line1_start.y)
    );

    u_a >= 0.0 && u_a <= 1.0 && u_b >= 0.0 && u_b <= 1.0
}

pub fn point_in_polygon(verticles: &[Point2<f32>], point: &Point2<f32>) -> bool {
    let point_end = Point2::new(INF, point.y);

    false
}

pub fn point_in_rect(rect: &Rect, point: &Point2<f32>) -> bool {
    point.x > rect.x && point.y > rect.x && point.x < rect.x + rect.w && point.y < rect.y + rect.h
}