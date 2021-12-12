#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Guess {
    value: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            //panic!("Guess value must be greater than or equal to 1, got {}", value);
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        else if value > 100 {
            //panic!("Guess value must be less than or equal to 100, got {}", value);
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    //String::from("Hello!")
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("This will fail the test.");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn this_adds_two(){
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn greeting_with_name(){
        let result = greeting("Carl");
        assert!(
            result.contains("Carl"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn generic_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
