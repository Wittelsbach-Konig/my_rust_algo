/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = Vec::new();

    for c in code.chars() {
        if !c.is_whitespace() {
            match c.to_digit(10) {
                Some(d) => sum.push(d),
                None => return false,
            }
        }
    }

    if sum.len() < 2 {
        return false;
    }
    sum.reverse();
    for i in (1..sum.len()).step_by(2) {
        let mut double = sum[i] * 2;
        if double > 9 {
            double -= 9;
        }
        sum[i] = double;
    }
    sum.iter().sum::<u32>() % 10 == 0
}