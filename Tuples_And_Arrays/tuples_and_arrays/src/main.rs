// Tuples and arrays are "compound" types
// All elements of an array have the same type, while tuples can accommodate different types.
// Both have a size fixed at compile time.
// 
//          Types                       Literals
// Arrays:  [T; N]                      [20, 30, 40], [0;3]
// Tuples:  (), (T, ), (T1, T2),...     (), ('x',), ('x', 1.2),...
//
// Arrays
// - A value of the array type `[T; N]` holds `N` (a compile-time constant) elements of the same
// type `T`. Note that the length of the array is part of its type, which means that `[u8; 3]` and
// `[u8; 4]` are considered two different types. Slices, which have a size determined at runtime,
// are covered later.
//
// Tuples
// - Tuples have fixed length
// - Tuples group together values of different types into a compound type
// - Fields of a tuple can be accessed by the period and the index of the value, e.g. `t.0`, `t.1`
// - The empty tuple `()` is also known as the "unit type". It is both a type, and the only valid
// value of that type - that is to say both the type and its value are expressed as `()`. It is
// used to indicate, for example, that a function or expression has no return value, as we'll see
// in a future slide - You can think of it as `void` that can be familiar to you from other programming languages.


fn main() {
    // Array assignment and access:
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    // Try accessing an out-of-bounds array element. Array accesses are checked at runtime. Rust
    // can usually optimize these checks away, and they can be avoided using unsafe Rust.
    // a[11] = 2;

    // `{:?}` is debug output
    // `{:#?}` invokes a "pretty printing" format
    println!("a: {a:?}");

    // Tuple assignment and access:
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}
