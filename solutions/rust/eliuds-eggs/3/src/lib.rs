pub fn egg_count(display_value: u32) -> usize {
    (0..32)
        .filter(|bit| display_value & (1 << bit) != 0)
        .count()
}
