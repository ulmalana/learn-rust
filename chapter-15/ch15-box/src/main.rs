use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

//define our own box
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//for deref coercion
fn hello(name: &str) {
    println!("Hello, {}.", name);
}

use crate::List::{Cons, Nil};

fn main() {
    //println!("Hello, world!");
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let y2 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y2);

    //deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
