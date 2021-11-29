#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    let user1 = User {
        username: String::from("username1"),
        email: String::from("user@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("{:?}", user1);

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };
    
    //println!("{:?}", user1); //this line will cause error because user1 is moved and not valid anymore

    println!("{:?}", user2);

    let user3 = build_user(
        String::from("someone@example.com"),
        String::from("user3"),
    );
    println!("{:?}", user3);

    println!("Hello, world!");
}
