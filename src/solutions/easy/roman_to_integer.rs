pub static MAX: i16 = 3999;
pub static MIN: i16 = 1;

static ROMAN_PAIRS: &[(i16, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

fn roman_map(c: &char) -> Option<i16> {
    match c {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None,
    }
}

pub fn roman_to_int(s: String) -> Option<i16> {
    let mut result = 0;
    let mut prev_value = 0;
    for ch in s.chars().rev() {
        let value = roman_map(&ch)?;
        if value < prev_value {
            result -= value;
        } else {
            result += value;
        }
        prev_value = value;
    }
    Some(result)
}

pub fn int_to_roman(mut num: i16) -> Option<String> {
    if num < MIN || num > MAX {
        return None;
    }
    let mut result = String::new();

    for &(value, symbol) in ROMAN_PAIRS.iter() {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }

    Some(result)
}
