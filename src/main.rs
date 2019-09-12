#![allow(dead_code)]
use std::f64::consts::PI;
use core::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    formatting();
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
