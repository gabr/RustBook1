//! The `testlib` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, testlib::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use testlib::add_two;
///
/// assert_eq!(4, add_two(2));
/// // $ RUST_TEST_NOCAPTURE=1 cargo test   # Preserve stdout/stderr
/// // $ cargo test -- --nocapture          # Same as above
/// println!("Testing print");
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_works() {
        assert_eq!(4, add_two(2));
        println!("Print test - math_works()");
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
