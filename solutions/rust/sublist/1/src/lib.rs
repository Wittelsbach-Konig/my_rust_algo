use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) => match m.cmp(&n) {
            Ordering::Equal if first_list == second_list => Comparison::Equal,
            Ordering::Less if is_sublist(first_list, second_list) => Comparison::Sublist,
            Ordering::Greater if is_sublist(second_list, first_list) => Comparison::Superlist,
            _ => Comparison::Unequal,
        },
    }
}

fn is_sublist<T: PartialEq>(short: &[T], long: &[T]) -> bool {
    if short.is_empty() {
        return true;
    }
    
    long.windows(short.len()).any(|window| window == short)
}
