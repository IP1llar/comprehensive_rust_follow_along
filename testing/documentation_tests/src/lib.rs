/// This is documentation
///
/// Code blocks in `///` comments are automatically seen as Rust code
/// The code will be compiled as part of `cargo test`
/// Adding `#` in the code will hide it from the docs, but will still compile/run it:
///
/// ```
/// # use documentation_tests::double;
/// assert_eq!(double(2), 4);
/// ```
pub fn double(x: u32) -> u32 {
    x * 2
}
