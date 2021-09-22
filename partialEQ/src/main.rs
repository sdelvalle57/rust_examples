

use std::cmp::PartialEq;

//#[derive(PartialEq)]
enum Foo {Bar, Zoo}

impl PartialEq for Foo {
    fn eq(&self, other: &Foo) -> bool {
        match (self, other) {
            (&Foo::Bar, &Foo::Bar) => true,
            (&Foo::Zoo, &Foo::Zoo) => true,
            _ => false
        }
    }
}

fn main() {
    let a = Foo::Bar;
    let b = Foo::Zoo;

    if Foo::Bar == a {
        println!("a is goobar")
    }

    if Foo::Bar == b {
        println!("b is goobar")
    } else {
        println!("b is nope")
    }

    if let Foo::Bar = a {
        println!("b is goobar2")
    }
}
