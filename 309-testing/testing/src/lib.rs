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

    #[test]
    #[should_panic]

    fn it_fails() {
        panic!("Error Error");
    }

    #[test]

    fn should_call_check_addition() {
        assert!(check_addition());
    }
}

fn check_addition() -> bool {
    if 2 + 2 == 4 { true } else { false }
}
