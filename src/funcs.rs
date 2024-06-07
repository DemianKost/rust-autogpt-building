/// Function: add_five
/// # Returns u32
/// # Arguments (num: u32)
/// # Example
/// ```
/// let x = 5;
/// let y = add_five(x);
/// ```
pub fn add_five(num: u32) -> u32 {
    return num + 5
}

// Unit test for test setup
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_five_test() {
        let x = 100;
        let y = add_five(x);
        println!("Result {}", y);
        assert_eq!(y, 105);
    }
}