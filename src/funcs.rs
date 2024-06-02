pub fn add_five(num: u32) -> u32 {
    return num + 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_five_test() {
        let x = 100;
        let y = add_five(x);
        println!("Result {}", y);
    }
}