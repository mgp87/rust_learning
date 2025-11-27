// cargo test -- --test-threads=n --> uses n threads for test execution
// cargo test test_function_name --> runs only the test function
// cargo test it --> runs the ones starting with it_

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn simple_add() -> bool {
    2 + 2 == 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 5); // not equals
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        panic!("Test failed");
    }

    #[test]
    fn call_simple_add() {
        assert!(simple_add());
    }
}
