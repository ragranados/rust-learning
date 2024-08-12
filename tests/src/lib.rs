pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("F".to_string())
        }
    }

    #[test]
    #[should_panic(expected = "Esto deberia fallar")]
    fn failing_test() {
        panic!("Esto deberia fallar si?");
    }

    #[test]
    #[ignore]
    fn expensive_test() {}
}
