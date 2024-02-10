fn main() {
    // Rust provides type safety via static typing.
    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");
}

// Values
// Signed integers:            i8, i16, i32, i64, i128, isize   | -10, 0, 1_000, 123_i64
// Unsigned integers:          u8, u16, u32, u64, u128, usize   | 0, 123, 10_u16
// Floating point numbers:     f32, f64                         | 3.14, -10.0e20, 2_f32
// Unicode scalar values:      char                             | 'a', '*'
// Booleans:                   bool                             | true, false
//
// The types have widths as follows:
// - iN, uN, and fN are N bits wide
// - isize and usize are the width of a pointer
// - char os 32 bits wide,
// - bool is 8 bits wide
