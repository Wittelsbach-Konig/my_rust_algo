pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in string.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                if let Some(open) = stack.pop() {
                    let matches = matches!((open, ch), ('(', ')') | ('[', ']') | ('{', '}'));
                    if !matches {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}
