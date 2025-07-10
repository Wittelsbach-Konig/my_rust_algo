pub fn collatz(mut n: u64) -> Option<u64> {
    for i in 0.. {
        match n {
            0 => break,
            1 => return Some(i),
            even if even & 1 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn zero_steps_for_one() {
        let output = collatz(1);
        let expected = Some(0);
        assert_eq!(output, expected);
    }
    #[test]
    fn divide_if_even() {
        let output = collatz(16);
        let expected = Some(4);
        assert_eq!(output, expected);
    }
    #[test]
    fn even_and_odd_steps() {
        let output = collatz(12);
        let expected = Some(9);
        assert_eq!(output, expected);
    }
    #[test]
    fn large_number_of_even_and_odd_steps() {
        let output = collatz(1_000_000);
        let expected = Some(152);
        assert_eq!(output, expected);
    }
    #[test]
    fn zero_is_an_error() {
        let output = collatz(0);
        let expected = None;
        assert_eq!(output, expected);
    }
}
