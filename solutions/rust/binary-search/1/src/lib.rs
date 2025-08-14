pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if &key < array.first()? {
        return None;
    }
    if &key > array.last()? {
        return None;
    }
    let mid = array.len() / 2;
    let middle = array[mid];
    if middle == key {
        return Some(mid);
    } else {
        binary_search(array, 0, array.len(), key)
    }
}

fn binary_search(array: &[i32], mut left: usize, mut right: usize, key: i32) -> Option<usize> {
    while left <= right {
        let mid = (left + right) / 2;
        let middle = array[mid];
        if middle == key {
            return Some(mid);
        }
        if middle > key {
            right = mid - 1;
        } else {
            left = mid + 1
        }
    }
    return None;
}
