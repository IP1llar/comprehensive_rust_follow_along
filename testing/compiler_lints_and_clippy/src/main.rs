// The Rust compiler produces fantastic error messages, as well as helpful built-in lints
// `Clippy` provides even more lints, organized into groups that can be enable per-project
#[deny(clippy::cast_possible_truncation)]
fn main() {
    let mut x = 3;
    while x < 70000 {
        x *= 2;
    }
    println!("X probably fits in a u16, right? {:?}", u16::try_from(x));
}
