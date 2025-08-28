/// Adds left and right numbers.
///
/// # Arguments
///
/// * `left` - The left operand.
/// * `right` - The right operand.
///
/// # Examples
///
/// ```
/// # use my_library::add; // Assuming the crate's name is `my_library`
/// let l: usize = 20;
/// let r: usize = 5;
/// assert_eq!(add(l, r), 25);
/// ```

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
