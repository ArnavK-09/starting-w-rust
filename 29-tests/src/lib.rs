/// Docs tests
///  ```
/// assert!(true);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 2)
    }

    #[test]
    #[ignore]
    fn test2() {
        // Adding Custom Failure Messages
        assert!(false, "hello world 67676767");
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }
}

// Running Tests in Parallel or Consecutively
// $ cargo test -- --test-threads=1

// Show println! output for passing tests too
// $ cargo test -- --show-output

// Run a single test by name
// $ cargo test it_works

// Run tests whose names contain a substring
// $ cargo test add

// Run only ignored tests
// $ cargo test -- --ignored

// Run all tests (including ignored)
// $ cargo test -- --include-ignored
