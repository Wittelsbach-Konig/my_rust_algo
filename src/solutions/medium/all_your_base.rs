#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn require<T>(predicate: bool, err: T) -> Result<(), T> {
    if predicate { Ok(()) } else { Err(err) }
}

fn convert_from_base(number: &[u32], base: u32) -> Result<u32, Error> {
    require(base >= 2, Error::InvalidInputBase)?;
    let mut result: u32 = 0;
    for &digit in number {
        require(digit < base, Error::InvalidDigit(digit))?;
        result = result * base + digit;
    }
    Ok(result)
}

fn convert_to_base(mut number: u32, base: u32) -> Result<Vec<u32>, Error> {
    require(base >= 2, Error::InvalidOutputBase)?;
    if number == 0 {
        return Ok(vec![0]);
    }
    let mut result: Vec<u32> = Vec::new();
    while number > 0 {
        result.push(number % base);
        number /= base;
    }
    result.reverse();
    Ok(result)
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    convert_from_base(number, from_base).and_then(|n| convert_to_base(n, to_base))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_ok() {
        assert!(require(true, Error::InvalidInputBase).is_ok());
    }

    #[test]
    fn test_require_err() {
        assert_eq!(
            require(false, Error::InvalidInputBase),
            Err(Error::InvalidInputBase)
        );
    }

    #[test]
    fn test_convert_from_base_binary() {
        let input = vec![1, 0, 1, 0];
        assert_eq!(convert_from_base(&input, 2), Ok(10));
    }

    #[test]
    fn test_convert_from_base_decimal() {
        let input = vec![1, 2, 3];
        assert_eq!(convert_from_base(&input, 10), Ok(123));
    }

    #[test]
    fn test_convert_from_base_invalid_base() {
        let input = vec![1, 0];
        assert_eq!(convert_from_base(&input, 1), Err(Error::InvalidInputBase));
    }

    #[test]
    fn test_convert_from_base_invalid_digit() {
        let input = vec![1, 2];
        assert_eq!(convert_from_base(&input, 2), Err(Error::InvalidDigit(2)));
    }

    #[test]
    fn test_convert_to_base_binary() {
        assert_eq!(convert_to_base(10, 2), Ok(vec![1, 0, 1, 0]));
    }

    #[test]
    fn test_convert_to_base_decimal() {
        assert_eq!(convert_to_base(123, 10), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_convert_to_base_zero() {
        assert_eq!(convert_to_base(0, 2), Ok(vec![0]));
    }

    #[test]
    fn test_convert_to_base_invalid_base() {
        assert_eq!(convert_to_base(10, 1), Err(Error::InvalidOutputBase));
    }

    #[test]
    fn test_round_trip_binary() {
        let original = vec![1, 0, 1, 1, 0];
        let base = 2;
        let decimal = convert_from_base(&original, base).unwrap();
        let back = convert_to_base(decimal, base).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_round_trip_hexadecimal() {
        let original = vec![15, 10, 5]; // F A 5
        let base = 16;
        let decimal = convert_from_base(&original, base).unwrap();
        let back = convert_to_base(decimal, base).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn single_bit_one_to_decimal() {
        let input_base = 2;
        let input_digits = &[1];
        let output_base = 10;
        let output_digits = vec![1];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn binary_to_single_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1];
        let output_base = 10;
        let output_digits = vec![5];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn single_decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[5];
        let output_base = 2;
        let output_digits = vec![1, 0, 1];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn binary_to_multiple_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[4, 2];
        let output_base = 2;
        let output_digits = vec![1, 0, 1, 0, 1, 0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn trinary_to_hexadecimal() {
        let input_base = 3;
        let input_digits = &[1, 1, 2, 0];
        let output_base = 16;
        let output_digits = vec![2, 10];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn hexadecimal_to_trinary() {
        let input_base = 16;
        let input_digits = &[2, 10];
        let output_base = 3;
        let output_digits = vec![1, 1, 2, 0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn test_15_bit_integer() {
        let input_base = 97;
        let input_digits = &[3, 46, 60];
        let output_base = 73;
        let output_digits = vec![6, 10, 45];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn empty_list() {
        let input_base = 2;
        let input_digits = &[];
        let output_base = 10;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn single_zero() {
        let input_base = 10;
        let input_digits = &[0];
        let output_base = 2;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn multiple_zeros() {
        let input_base = 10;
        let input_digits = &[0, 0, 0];
        let output_base = 2;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn leading_zeros() {
        let input_base = 7;
        let input_digits = &[0, 6, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn input_base_is_one() {
        let input_base = 1;
        let input_digits = &[0];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidInputBase)
        );
    }
    #[test]
    fn input_base_is_zero() {
        let input_base = 0;
        let input_digits = &[];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidInputBase)
        );
    }
    #[test]
    fn invalid_positive_digit() {
        let input_base = 2;
        let input_digits = &[1, 2, 1, 0, 1, 0];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidDigit(2))
        );
    }
    #[test]
    fn output_base_is_one() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 1;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidOutputBase)
        );
    }
    #[test]
    fn output_base_is_zero() {
        let input_base = 10;
        let input_digits = &[7];
        let output_base = 0;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidOutputBase)
        );
    }
}
