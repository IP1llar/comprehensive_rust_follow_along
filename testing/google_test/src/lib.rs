// The GoogleTest crate allows for flexible test assertions using matchers:
use googletest::prelude::*;

#[googletest::test]
fn test_elements_are() {
    let value = vec!["foo", "bar", "baz"];
    expect_that!(value, elements_are!(eq("foo"), lt("xyz"), starts_with("b")));
}

// If we change the last element to `!`, the test fails with a structured error message
// pin-pointing the error
//
//
// Google test needs to be added to cargo projects
//
// The `use googletest::prelude::*;` line imports a number of commonly used macros and types
//
// This just scratches the surface, there are many builtin matchers
//
// A particularly nice feature is that mismatches in multi-line strings are shown as a color-coded diff 
#[test]
fn test_multiline_string_diff() {
    let haiku = "Memory safety found,\n\
                 Rust's strong typing guides the way,\n\
                 Secure code you'll write.";
    assert_that!(
        haiku,
        eq("Memory safety found,\n\
            Rust's silly humor guides the way,\n\
            Secure code you'll write.")
    );
}
// The crate is a Rust port of GoogleTest for C++
// GoogleTest is available for use in AOSP
