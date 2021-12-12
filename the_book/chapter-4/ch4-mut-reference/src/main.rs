fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    change(&mut s);

    println!("changed s: {}", s);
}

fn change(some_string: &mut String){
    some_string.push_str(", world.")
}
