use ggez::graphics;
use ggez::nalgebra::{Point2};

const INF: f32 = 1000000.0;

// let s: Box<[i32]> = Box::new([10, 40, 30]);
// let x = s.into_vec();

// pub struct CollisionShapePolygon {
//     points: Vec<Point2<f32>>
// }

// pub struct CollisionShapeRect {
//     rect: graphics::Rect
// }

// trait Collidable {
//     fn intersect_point(point: &Point2<f32>) -> bool;
//     fn intersect_line(line_start: &Point2<f32>, line_end: &Point2<f32>) -> bool;
// }


pub fn line_intersection(line1_start: &Point2<f32>, line1_end: &Point2<f32>, line2_start: &Point2<f32>, line2_end: &Point2<f32>) -> bool {
    // float uA = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
    let uA: f32 = (
        (line2_end.x-line2_start.x)*(line1_start.y-line2_start.y) -
        (line2_end.y-line2_start.y)*(line1_start.x-line2_start.x)
    ) /
    (
        (line2_end.y-line2_start.y)*(line1_end.x-line1_start.x) -
        (line2_end.x-line2_start.x)*(line1_end.y-line1_start.y)
    );

    // float uB = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
    let uB: f32 =
    (
        (line1_end.x-line1_start.x)*(line1_start.y-line2_start.y) -
        (line1_end.y-line1_start.y)*(line1_start.x-line2_start.x)
    ) /
    (
        (line2_end.y-line2_start.y)*(line1_end.x-line1_start.x) -
        (line2_end.x-line2_start.x)*(line1_end.y-line1_start.y)
    );

    uA >= 0.0 && uA <= 1.0 && uB >= 0.0 && uB <= 1.0
}

pub fn point_in_polygon(verticles: &[Point2<f32>], point: &Point2<f32>) -> bool {
    let point_end = Point2::new(INF, point.y);

    false
}

pub fn point_in_rect(rect: &graphics::Rect, point: &Point2<f32>) -> bool {
    point.x > rect.x && point.y > rect.x && point.x < rect.x + rect.w && point.y < rect.y + rect.h
}