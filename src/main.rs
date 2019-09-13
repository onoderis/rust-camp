#![allow(dead_code)]
use std::f64::consts::PI;
use core::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    structures();
}


// 3.1. Structures
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
    let mut width = rect.p1.x - rect.p2.x;
    if width < 0.0 {
        width = width * -1.0;
    };


    let mut height = rect.p1.y - rect.p2.y;
    if height < 0.0 {
        height = height * -1.0;
    };

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

fn structures() {
    let point: Point = Point { x: 1.0, y: 1.0 };
    let rectangle = Rectangle {
        p1: Point { x: 3.0, y: 3.0 },
        p2: point,
    };
    println!("rect_area: {}", rect_area(rectangle));

    let point2: Point = Point { x: 1.0, y: 1.0 };
    println!("square:\n{}", square(point2, 3.0));
}


// 2.2. Tuples
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn tuples() {
    let matrix = Matrix(
        1.1, 1.2,
        2.1, 2.2
    );
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(
        m.0, m.2,
        m.1, m.3
    )
}


// 1.2.3. Formatting
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn formatting() {
    let colors = [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ];
    for color in colors.iter() {
        println!("{}", *color);
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)?;
        write!(f, " ")?;
        write!(f, "0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}


// 1.2.2.1. Testcase: List
fn testcase_list() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{count}: {value}", count = count, value = v)?;
        }
        write!(f, "]")
    }
}


// 1.2. Formatted print
fn formatted_print() {
    println!("{:.*}", 3, PI)
}
