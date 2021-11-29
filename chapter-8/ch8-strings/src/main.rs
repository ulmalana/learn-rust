fn main() {
    let mut s = String::new();
    s = "string 1".to_string();
    s.push_str("-up");

    println!("{}", s);

    let mut s2 = String::from("lo");
    s2.push('l');

    println!("{}", s2);

    //concat strings
    let s3 = String::from("hello");
    let s4 = String::from("world.");
    let s5 = s3 + &s4;
    println!("{}", s5);

    //format macro
    let s7 = String::from("guten");
    let s8 = String::from("tag.");
    let s9 = format!("{} {}", s7, s8);
    println!("{}", s9);
}
