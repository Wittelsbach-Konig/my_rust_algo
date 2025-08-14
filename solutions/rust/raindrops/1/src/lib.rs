pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    if is_divisible_by_3(n) {
        result.push_str("Pling");
    }
    if is_divisible_by_5(n) {
        result.push_str("Plang");
    }
    if is_divisible_by_7(n) {
        result.push_str("Plong");
    }
    if result.is_empty() {
        result = n.to_string();
    }
    result
}

fn is_divisible_by_3(n: u32) -> bool {
    let q = ((n as u64 * 0xAAAAAAAB_u64) >> 33) as u32;
    q * 3 == n
}

fn is_divisible_by_5(n: u32) -> bool {
    let q = ((n as u64 * 0xCCCCCCCD_u64) >> 34) as u32;
    q * 5 == n
}

fn is_divisible_by_7(n: u32) -> bool {
    // 2^32 / 7 = 613566757
    // M = ceil(2^32 / 7) = 613566757
    // s = 32
    let m = 613566757_u64;
    let q = ((n as u64 * m) >> 32) as u32;
    q * 7 == n
}