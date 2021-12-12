fn main() {
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2 = vec![4,5,6];
    println!("Hello, world!");

    let second: &i32 = &v1[1];
    println!("The second element of v2 is {}", second);

    for i in &mut v1{
        println!("{}", (*i)*10);
    }

    for i in &v2{
        println!("{}", i);
    }

    //v1 stay the same
    for i in &v1{
        println!("{}", i);
    }
}
