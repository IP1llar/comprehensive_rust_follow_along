fn main() {
    println!("gcd: {}", gcd(143, 52));
}

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

// Overloading is not supported - each function has a single implementation
// - Always takes a fixed number of parameters. Default arguments are not supported. Macros can be
// used to support variadic functions.
// - Always takes a single set of parameter types. These types can be generic.
