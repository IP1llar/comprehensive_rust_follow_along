// While move semantics are the default, certain types are copied by default - these types
// implement the `Copy` trait:
fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}"); // would not be accessible if not `Copy`
    println!("y: {y}");

    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}"); // Would not be accessible if not `Copy`
    println!("p2: {p2:?}");
}

// You can opt your own types to use copy semantics by having them implement the `Copy` trait:
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

// Cloning and Copying are not the same thing:
// - Copying refers to bitwise copies of memory regions and does not work on arbitrary objects
// - Copying does not allow for custom logic (unlike copy constructors in C++)
// - Cloning is a more general operation and also allows for custom behavior by implementing the
// `Clone` trait
// - Copying does not work on types that implement the `Drop` trait
//
// Adding a `String` to `Point` will not compile because `String` is not a `Copy` type
