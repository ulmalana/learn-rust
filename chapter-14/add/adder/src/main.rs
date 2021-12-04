use add_one;
use rand;
use add_two;

fn main() {
    //println!("Hello, world!");
    let num = 10;
    println!("{} plus one is {}", num, add_one::add_one(num));
    println!("{} plus two is {}", num, add_two::add_two(num));
}
