#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//method syntax
impl Rectangle {
    //using self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //without using self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect2 = Rectangle::square(5);
    println!(
        "The area of the rect1 is {} square units",
        rect1.area()
        );
    println!(
        "The area of the rect2 is {} square units",
        rect2.area()
        );
}
