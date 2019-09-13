use std::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    let c = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", c);
    println!("Debug: {:?}", c);
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}
