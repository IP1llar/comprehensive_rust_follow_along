// A type is `Sync` if it is safe to access a `T` value from multiple threads at the same time
//
// More precisely, the definition is:
// `T` is `Sync` if and only if `&T` is `Send`
//
// This statement is essentially a shorthand way of saying that if a type is thread-safe for shared
// use, it is also thread-safe to pass references of it across threads.
//
fn main() {
    println!("Hello, world!");
}
