#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = "klm";
    let string3 = String::from("short string");

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}.", result);

    let result = longest(string1.as_str(), string3.as_str());
    println!("The longest string is {}.", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("first sentence: {}", excerpt.part);
}
