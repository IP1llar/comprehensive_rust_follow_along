// How does Rust know to forbid shared access across threads? The answer is in two traits:
// - `Send`: a type `T` is `Send` if it is safe to move a `T` across a thread boundary
// - `Sync`: a type `T` is `Sync` if it is safe to move a `&T` across a thread boundary
//
// `Send` and `Sync` are unsafe traits. The compiler will automatically derive them for your types
// as long as they only contain `Send` and `Sync` types. You can also implement them manually when
// you know it is valid
//
// One can think of these traits as markers that the type has certain thread-safety properties
// They can be used in the generic constraints as normal traits
fn main() {
    println!("Hello, world!");
}
