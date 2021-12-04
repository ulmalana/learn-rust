#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = 2 + 2;
        assert_eq!(add_one(2), 3);
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
