fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn main() {
    let num_list = vec![34, 50, 25, 95, 100, 75];
    let result = largest(&num_list);
    println!("The largest number is {}.", result);

    let char_list = vec!['f', 'y', 'm', 'a', 'z', 'd'];
    let result = largest(&char_list);
    println!("The largest char is {}.", result);
    //println!("Hello, world!");
}
