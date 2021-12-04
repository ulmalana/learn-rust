pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add_two() {
        assert_eq!(add_two(2), 4);
    }
}
