pub fn add_two(n: i32) -> i32{
    n + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, add_two(3));
    }
}
