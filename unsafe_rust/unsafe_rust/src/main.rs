// The Rust language has two parts:
// - Safe Rust: memory safe, no undefined behavior possible
// - Unsafe Rust: can trigger undefined behavior if preconditions are violated
//
// Unsafe code is usually small and isolated, and its correctness should be carefully documented.
// It is usually wrapped in a safe abstraction layer
//
// Unsafe Rust gives you access to five new capabilities:
// - Dereference raw pointers
// - Access or modify mutable static variables
// - Access `union` fields
// - Call `unsafe` functions, including `extern` functions
// - Implement `unsafe` traits
fn main() {
    println!("Hello, world!");
}
