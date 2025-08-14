pub fn reverse(input: &str) -> String {
    let inverse_input: String = input.to_string().chars().rev().collect();
    inverse_input
}
