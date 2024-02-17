// If you want to test your library as a client, use an integration test
// Create a `.rs` file uner `tests/`
pub fn double(x: u32) -> u32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_number_in_librs() {
        assert_eq!(double(2), 4);
    }
}
