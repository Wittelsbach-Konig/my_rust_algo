pub fn egg_count(mut display_value: u32) -> usize {
    (0..32)
        .filter(|bit| display_value & (1 << bit) != 0)
        .count()
}
