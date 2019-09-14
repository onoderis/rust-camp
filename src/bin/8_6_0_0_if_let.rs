fn main() {
    let a = Foo::Bar;

    if a == Foo::Bar {
        println!("a is foobar");
    }
}

#[derive(PartialEq)]
enum Foo { Bar }
