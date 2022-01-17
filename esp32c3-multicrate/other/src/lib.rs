pub fn my_fun() -> i32 {
    println!("Hello, other!");
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(42, my_fun(), "m");
    }

    #[test]
    fn another() {}
}