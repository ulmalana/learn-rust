fn main() {
    let x = five();

    let y = add_five(3);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn add_five(x: i32) -> i32 {
    x + 5
}
