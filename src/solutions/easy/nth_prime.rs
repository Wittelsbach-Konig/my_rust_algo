pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 2;
    while count < n {
        num += 1;
        if is_prime(num) {
            count += 1;
        }
    }
    num
}

fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        n if n & 1 == 0 => false,
        _ => (3..=n.isqrt()).step_by(2).all(|i| !n.is_multiple_of(i)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_prime() {
        let output = nth(0);
        let expected = 2;
        assert_eq!(output, expected);
    }
    #[test]
    fn second_prime() {
        let output = nth(1);
        let expected = 3;
        assert_eq!(output, expected);
    }
    #[test]
    fn sixth_prime() {
        let output = nth(5);
        let expected = 13;
        assert_eq!(output, expected);
    }
    #[test]
    fn big_prime() {
        let output = nth(10_000);
        let expected = 104_743;
        assert_eq!(output, expected);
    }
}
