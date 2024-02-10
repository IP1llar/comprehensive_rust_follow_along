// Rust comes with extensive documentation. For example:
// - All of the details about loops: https://doc.rust-lang.org/stable/reference/expressions/loop-expr.html
// - Primitive types like `u8`: https://doc.rust-lang.org/stable/std/primitive.u8.html
// - Standard library types like Option or Binary Heap: https://doc.rust-lang.org/stable/std/option/enum.Option.html or https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html
//
// You can document your own code:

/// Determine whether the first argument is divisibe by the second argument
///
/// If the second argument is zero, the result is false.
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

// The contents are treated as Markdown. All published Rust library crates are automatically
// documented at `docs.rs` using the `rustdoc` tool



fn main() {
    println!("{} is divisible by {}: {}", 25, 5, is_divisible_by(25, 5));
    println!("Hello, world!");
}
