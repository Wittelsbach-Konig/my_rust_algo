pub mod imperative {
    pub fn is_armstrong_number(num: u32) -> bool {
        let num_len = num.checked_ilog10().unwrap_or(0) + 1;
        let mut result: u32 = 0;
        let mut _num = num;
        for _ in 0..num_len {
            result += (_num % 10).pow(num_len);
            _num /= 10;
        }
        result == num
    }
}

pub mod functional {
    pub fn is_armstrong_number(num: u32) -> bool {
        num.to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap().pow(num.to_string().len() as u32))
            .sum::<u32>()
            == num
    }
}

#[cfg(test)]
mod test_armstrong_numbers {
    fn common_tests<F>(is_armstrong_number: F)
    where
        F: Fn(u32) -> bool,
    {
        assert!(is_armstrong_number(0));
        assert!(is_armstrong_number(5));
        assert!(!is_armstrong_number(10));
        assert!(is_armstrong_number(153));
        assert!(!is_armstrong_number(100));
        assert!(is_armstrong_number(9474));
        assert!(!is_armstrong_number(9475));
        assert!(is_armstrong_number(9_926_315));
        assert!(!is_armstrong_number(9_926_314));
    }

    #[test]
    fn test_functional() {
        common_tests(super::functional::is_armstrong_number);
    }

    #[test]
    fn test_imperative() {
        common_tests(super::imperative::is_armstrong_number);
    }
}
