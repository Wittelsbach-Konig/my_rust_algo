//! Matching Brackets
//!
//! Verifies that all brackets in a string are properly matched and
//! nested. Supported bracket pairs are `()`, `[]`, and `{}`. Any other
//! characters are ignored.
//!
//! The function uses a stack to track opening brackets and ensures each
//! closing bracket corresponds to the most recent unmatched opener.
//!
//! - Returns `true` when brackets are balanced or no brackets are present.
//! - Returns `false` for mismatched, crossing, or unpaired brackets.
//!
//! Complexity: O(n) time and O(n) space in the worst case, where n is
//! the length of the input string.

/// Checks whether `string` contains balanced brackets.
///
/// Supported pairs: `()`, `[]`, `{}`. All other characters are ignored.
///
/// # Examples
///
/// Balanced input:
/// ```rust
/// use my_rust_algo::solutions::easy::matching_brackets::brackets_are_balanced;
/// assert!(brackets_are_balanced("{what is (42)}?"));
/// assert!(brackets_are_balanced("{[()]}"));
/// assert!(brackets_are_balanced("hello"));
/// ```
///
/// Not balanced:
/// ```rust
/// use my_rust_algo::solutions::easy::matching_brackets::brackets_are_balanced;
/// assert!(!brackets_are_balanced("[text}"));
/// assert!(!brackets_are_balanced("([)]"));
/// assert!(!brackets_are_balanced("("));
/// assert!(!brackets_are_balanced(")"));
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_is_balanced() {
        assert!(brackets_are_balanced(""));
    }

    #[test]
    fn no_brackets_is_balanced() {
        assert!(brackets_are_balanced("hello, world! 123"));
    }

    #[test]
    fn simple_pairs_are_balanced() {
        assert!(brackets_are_balanced("()"));
        assert!(brackets_are_balanced("[]"));
        assert!(brackets_are_balanced("{}"));
    }

    #[test]
    fn nested_brackets_are_balanced() {
        assert!(brackets_are_balanced("{[()]}"));
        assert!(brackets_are_balanced("[({})]"));
        assert!(brackets_are_balanced("({[]})"));
    }

    #[test]
    fn intermixed_text_is_ignored() {
        assert!(brackets_are_balanced("{what is (42)}?"));
        assert!(brackets_are_balanced("abc(def)[ghi]{jkl}"));
    }

    #[test]
    fn mismatched_closing_fails() {
        assert!(!brackets_are_balanced("[text}"));
        assert!(!brackets_are_balanced("(]"));
        assert!(!brackets_are_balanced("{]"));
    }

    #[test]
    fn crossing_brackets_fail() {
        assert!(!brackets_are_balanced("([)]"));
        assert!(!brackets_are_balanced("{[}]"));
    }

    #[test]
    fn unmatched_openers_fail() {
        assert!(!brackets_are_balanced("("));
        assert!(!brackets_are_balanced("["));
        assert!(!brackets_are_balanced("{"));
        assert!(!brackets_are_balanced("(({}"));
    }

    #[test]
    fn unmatched_closers_fail() {
        assert!(!brackets_are_balanced(")"));
        assert!(!brackets_are_balanced("]"));
        assert!(!brackets_are_balanced("}"));
        assert!(!brackets_are_balanced("())"));
    }

    #[test]
    fn unsupported_symbols_are_ignored() {
        // Angle brackets are not part of the problem set; ignored -> balanced
        assert!(brackets_are_balanced("<>"));
        assert!(brackets_are_balanced("<a(b)[c]{d}>"));
    }
}
