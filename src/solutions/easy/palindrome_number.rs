
pub fn is_palindrome(x: i32) -> bool {
    let str_x = x.to_string();
    let inverse_x: String = str_x.chars().rev().collect();
    inverse_x == str_x
}
