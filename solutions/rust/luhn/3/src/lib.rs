/// The function `is_valid` in Rust checks if a given code passes the Luhn checksum algorithm.
///
/// Arguments:
///
/// * `code`: The code you provided is a function that checks if a given code follows the Luhn algorithm
/// for checksum validation. The Luhn algorithm is used to validate a variety of identification numbers,
/// such as credit card numbers.
///
/// Returns:
///
/// The function `is_valid` returns a boolean value indicating whether the input code passes the Luhn
/// checksum validation. If the code passes the Luhn checksum validation, the function returns `true`,
/// otherwise it returns `false`.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count & 1 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (sum + num, count + 1))
        })
        .is_some_and(|(sum, count)| sum % 10 == 0 && count > 1)
}