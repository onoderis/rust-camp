use core::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    let colors = [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ];
    for color in colors.iter() {
        println!("{}", *color);
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)?;
        write!(f, " ")?;
        write!(f, "0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
