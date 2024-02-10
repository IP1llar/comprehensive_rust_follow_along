// An assignment will transfer ownership between variables:
fn main() {
    let s1: String = String::from("Hello!");
    // The assignment of `s1` to `s2` transfers ownnership
    let s2: String = s1;
    println!("s2: {s2}");

    // When `s1` goes out of scope, nothing happens: it does not own anything
    // When `s2` goes out of scope, the string data is freed

    // When you pass a value to a function, the value is assigned to the function parameter.
    // - This transfers ownership

    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name) breaks
}

fn say_hello(name: String) {
    println!("Hello {name}");
}


// Before move to `s2`:
//
// Stack                Heap
//
//
// s1
// ------------------       ---------
// | ptr        | --------->|R|u|s|t|
// | len        | 4 |       ---------
// | capacity   | 4 |
// ------------------
//
// After move to `s2`:
//
// Stack                Heap
//
//
// s1
// ------------------       ---------
// | ptr        | --------->|R|u|s|t|
// | len        | 4 |   |   ---------
// | capacity   | 4 |   |
// ------------------   |
//                      |
// s2                   |
// ------------------   |    
// | ptr        | -------
// | len        | 4 |     
// | capacity   | 4 |
// ------------------
//
//
// Note that this is the opposite to the defaults in C++
// - which copies by value unless you use `std::move` (and the move constructor is defined)
//
// It is only the ownership value that moves - whether any machine code is generated to manipulate
// the data itself is a matter of optimization, and such copies are aggressively optimized away
//
// Simple values (such as integers) can be marked by `Copy`
//
// In Rust, clones are explicit (by using `Clone`)
//
//
// In the `say_hello` example:
// - With the first call to `say_hello`, `main` gives up ownership of `name`. Afterwards, `name`
// cannot be used anymore within `main`
// - The heap memory allocated for `name` will be freed at the end of the `say_hello` function
// - `main` can retain ownership if it passes `name` as a reference (`&name`) and if `say_hello`
// accepts a reference as a parameter
// - Alternatively, `main` can pass a clone of `name` in the first call (`name.clone()`)
// - Rust makes it harder than C++ to inadvertently create copies by making move semantics the
// default, and by forcing programmers to make clones explicit
