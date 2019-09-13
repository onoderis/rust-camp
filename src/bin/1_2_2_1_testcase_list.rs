use core::fmt;

fn main() {
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
