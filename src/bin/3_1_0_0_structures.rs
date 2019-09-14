use core::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    let point: Point = Point { x: 1.0, y: 1.0 };
    let rectangle = Rectangle {
        p1: Point { x: 3.0, y: 3.0 },
        p2: point,
    };
    println!("rect_area: {}", rect_area(rectangle));

    let point2: Point = Point { x: 1.0, y: 1.0 };
    println!("square:\n{}", square(point2, 3.0));
}


struct Point {
    x: f32,
    y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.p1, self.p2)
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    let width = f32::abs(rect.p1.x - rect.p2.x);
    let height = f32::abs(rect.p1.y - rect.p2.y);
    width * height
}

fn square(bottom_left_point: Point, width: f32) -> Rectangle {
    let top_right_x = bottom_left_point.x + width;
    let top_right_y = bottom_left_point.y + width;

    Rectangle {
        p1: bottom_left_point,
        p2: Point { x: top_right_x, y: top_right_y },
    }
}
