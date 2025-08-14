use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    for (&key, value) in h {
        for &c in value {
            result
                .entry(c.to_ascii_lowercase())
                .and_modify(|e| *e += key)
                .or_insert(key);
        }
    }
    result
}
