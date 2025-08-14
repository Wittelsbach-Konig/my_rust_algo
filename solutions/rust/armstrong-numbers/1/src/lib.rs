pub fn is_armstrong_number(num: u32) -> bool {
    let num_len = num.checked_ilog10().unwrap_or(0) + 1;
    let mut result: u32 = 0;
    let mut _num = num;
    for _ in 0..num_len {
        result += (_num % 10).pow(num_len);
        _num /= 10;
    }
    if result == num {
        return true;
    }
    false
}
