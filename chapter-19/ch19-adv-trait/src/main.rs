use std::ops::Add;
use std::fmt;


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,            
        }

    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!.");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

////////////////////////////////

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


/////////////////////////////////
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len+4));
        println!("*{}*", " ".repeat(len+2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len+2));
        println!("{}", "*".repeat(len+4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//////////////////////////
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    //println!("Hello, world!");
    assert_eq!(
        Point{x:1, y: 0} + Point{x: 2, y: 3},
        Point {x: 3, y: 3} 
    );

    //same name methods. call the method directly implemente in type
    let person = Human;
    person.fly(); //print *waving arms furiously*
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("1. A baby dog is called a {}", Dog::baby_name()); //will print spot

    //fully qualified syntax
    println!("2. A baby dog is called a {}", <Dog as Animal>::baby_name()); //will print puppy

    let p1 = Point {x: 5, y: 6};
    p1.outline_print();


    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
