#![allow(dead_code)]
use std::f64::consts::PI;
use core::fmt;

fn main() {
    testcase_list();
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
