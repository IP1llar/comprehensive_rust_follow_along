// Rust has no implicit type conversions, but does support explicit cases with `as`
// - These generally follow C semantics where those are defined
fn main() {
    let value: i64 = -1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);
}

// The results of `as` are always defined in Rust and consistent across platforms
// - This may not match your intuition for changing sign or casting to a smaller type (check docs
// and comment for clarity)
//
// Casting with `as` is a relatively sharp tool that is easy to use incorrectly, and can be a
// source of subtle bugs as future maintenance work changes the types that are used or the ranges
// of values in types
// Casts are best used only when the intent is to indicate unconditional truncation (e.g. selecting
// the bottom 32 bits of a `u64` with `as u32`, regardless of what was in the high bits)
//
// For infallible (e.g. `u32` to `u64`), prefer using `From` or `Into` over `as` to confirm that
// the cast is in fact infallibe. For fallible casts, `TryFrom` and `TryInto` are available when
// you want to handle casts that fit differently from those that don't
