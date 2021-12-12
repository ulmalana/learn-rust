fn main() {
    //println!("Hello, world!");
    let v1: Vec<i32> = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("Vec: {:?}", v1);
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("Vec: {:?}", v1);
    println!("Vec: {:?}", v2);
}
