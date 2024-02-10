// A slice gives you a view into a larger collection:
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    // The following code prevents compilation as `a` is already borrowed in `s`
    // a[3] = 4;

    println!("s: {s:?}");
}
// We create a slice by borrowing `a` and specifying the starting and ending indexes in brackets
//
// If the slice starts at index 0, Rust's range syntax allows us to drop the starting index,
// meaning that `&a[0..a.len()]` and `&a[..a.len()]` are identical
//
// The same is true for the last index, so `&a[2..a.len()]` and `&a[2..]` are identical
//
// To easily create a slice of the full array, we can therefore use `&a[..]`
//
// `s` is a reference to a slice of `i32`s. Notice that the type of `s` (`&[i32]`) no longer
// mentions the array length - This allows us to perform computation on slices of different sizes
//
// Slices always borrow from another object. In this example `a` has to remain 'alive' (in scope)
// for at least as our slice
//
// You cannot modify `a[3]` for memory safety reasons
// - You cannot do it through `a` at this point in the execution, but you can read the data from
// both `a` and `s` safely
// - It works before you created the slice, and again after the `println`, when the slice is no
// longer used
