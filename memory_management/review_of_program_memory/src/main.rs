// Programs allocate memory in two ways:
// 
// Stack: Continuous area of memory for local variables
// - Values have fixed sizes, known at compile time
// - Extremely fast: just move a stack pointer
// - Easy to manage: follows function calls
// - Great memory locality
//
// Heap: Storage of values outside of function calls
// - Values have dynamic sizes determined at runtime
// - Slightly slower than the stack: some book-keeping needed
// - No guarantee of memory locality
//
fn main() {
    // Creating a `String` puts a fixed-sized metadata on the stack and dynamically sized data, the
    // actual string, on the heap
    let mut s1 = String::from("Hello");
    // Stack                            Heap
    //
    // s1
    // --------------------------            -----------
    // |    ptr         |   ---------------> |H|e|l|l|o|
    // |    len         |   5   |            -----------
    // |    capacity    |   5   |
    // --------------------------
    //
    // A `String` is backed by a `Vec`, so it has a capacity and length and can grow if mutable via
    // reallocation on the heap
    // The underlying memory is heap allocated using the `System Allocator` and custom allocators
    // can be implemented using the `Allocator API`
    //
    // We can inspect the memory layout with `unsafe` Rust - this is unsafe!
    //
    s1.push(' ');
    s1.push_str("world");
    // DON'T DO THIS AT HOME! For educational purposes only.
    // `String` provides no guarantees about its layout, so this could lead to undefined behaviour
    println!("str: {s1}");
    unsafe {
        let (ptr, capacity, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }
}
