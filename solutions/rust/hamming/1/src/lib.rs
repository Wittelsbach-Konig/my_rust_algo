use std::cmp::Ordering;

pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    match s1.len().cmp(&s2.len()) {
        Ordering::Equal => Some(
            s1.chars()
                .zip(s2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count(),
        ),
        Ordering::Greater => None,
        Ordering::Less => None,
    }
}

