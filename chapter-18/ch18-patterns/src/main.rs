struct Point {
    x: i32,
    y: i32,
}

//@ bindings
enum Message {
    Hello {id: i32},
}

//pattern in fn
fn print_coordinates(&(x,y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    //println!("Hello, world!");

    //match pattern
    let fav_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_color {
        println!("Using your fav color, {}, as background.", color);
    }
    else if is_tuesday {
        println!("Tuesday is green day.");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background color");
        }
        else {
            println!("Using orange as background color");
        }
    }
    else {
        println!("Using blue as background color");
    }

    //while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //for pattern
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //pattern in fn
    let point = (3,5);
    print_coordinates(&point);

    //matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50."),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    //destructuring struct
    let p = Point {x: 0, y: 7};

    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x,y} => println!("On neither axis: at ({}, {})", x,y),
    }

    //@ bindings
    let msg = Message::Hello{id: 6};
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("found an id in range: {}", id_variable),
        Message::Hello {id: 10..=12} => {
            println!("Found an id in another range")
        }
        Message::Hello {id} => println!("found some other id: {}", id),
    }

}
