/// Subtracts ten from the given number.
/// This is how documentation is made in Rust

/**
 * This is another way
 */

/*
    This is yet another way
*/

// By executing "cargo doc", you can generate documentation for your Rust code from comments.
// The documentation will be generated in the target/doc directory.

pub fn substract_ten(x: u32) -> u32 {
    x - 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substract_ten() {
        assert_eq!(substract_ten(10), 0);
        assert_eq!(substract_ten(15), 5);
        assert_eq!(substract_ten(100), 90);
    }
}
