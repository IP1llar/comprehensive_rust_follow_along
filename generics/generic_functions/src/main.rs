// Rust supports generics
// You can abstract over the types used or stored
// Rust infers a type T based on the types of the arguments and return value
// This is similar to C++ templates, but Rust partially compiles the generic function immediately,
// so that function must be valid for all types matching the constraints
fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n % 2 == 0 {
        even
    } else {
        odd
    }
    // The following would be legal in C++, but Rust checks against all types matching the
    // constraints
    // return even + odd;
}

fn main() {
    println!("picked a number: {:?}", pick(97, 222, 333));
    println!("picked a tuple: {:?}", pick(28, ("dog", 1), ("cat", 2)));
}

// Generic code is turned into non-generic code based on the call sites
// This is a zero-cost abstraction: you get exactly the same result as if you had hand-coded the
// data structures without the abstraction
