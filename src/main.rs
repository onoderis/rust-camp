use std::f64::consts::PI;

fn main() {
    formatted_print_1_2();
}

fn formatted_print_1_2() {
    println!("{:.*}", 3, PI)
}
