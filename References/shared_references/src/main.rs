// Shared references are read-only, and the referenced data cannot change
fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}
// A shared reference to a type `T` has type `&T`
// A reference value is made with the `&` operator.
// The `*` operator "dereferences" a reference, yielding its value.
//
// A reference is said to "borrow" the value it refers to
// - code can use the reference to access the value, but is still "owned" by the original variable
//
// References are implemented as pointers
// Rust does not automatically create references for you - the `&` is always required
// Rust will auto-dereference in some cases, in particular when invoking methods
//
// Rust will statically forbid dangling references:
// fn x_axis(x: i32) -> &(i32, i32) {
//    let point = (x, 0);
//    return &point;
// }
// Rust is tracking the lifetimes of all references to ensure they live long enough
// Dangling references cannot occur in safe Rust
// `x_axis` would return a reference to `point`, but `point` will de deallocated when the function
// returns, so this will not compile.
//
