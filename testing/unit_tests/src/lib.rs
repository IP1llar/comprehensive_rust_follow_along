// Rust and Cargo come with a simple unit test framework:
// - Unit tests are supported throughout your code
// - Integration tests are supported via the `tests/` directory
//
// Tests are marked with `#[test]`
// Unit tests are often put in a nested `tests` module, using `#[cfg(test)]` to conditionally
// compile them only when building tests
pub fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(first_word("Hello"), "Hello");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(first_word("Hello World"), "Hello");
    }
}
