pub fn add_five(x: u32) -> u32 {
    x + 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_five() {
        assert_eq!(add_five(5), 10);
        assert_eq!(add_five(0), 5);
        assert_eq!(add_five(100), 105);
    }
}
