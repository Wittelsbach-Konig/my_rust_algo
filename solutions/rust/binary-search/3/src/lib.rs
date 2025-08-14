use std::cmp::Ordering;

pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let (mut left, mut right) = (0usize, array.len());
    let mut middle: usize;

    while left < right {
        middle = (left + right) / 2;
        match key.cmp(array.get(middle)?) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => {
                left = middle + 1;
            }
            Ordering::Less => right = middle,
        }
    }
    None
}

