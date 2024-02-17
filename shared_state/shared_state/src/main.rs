// Rust use the type system to enforce synchronization of shared data. This is primarily done via
// two types:
// - `Arc<T>`, atomix reference counted `T`: handles sharing between threads and takes care to
// deallocate `T` when the last reference is dropped,
// - `Mutex<T>`: ensure mutually exclusive access to the `T` value
//
fn main() {
    println!("Hello, world!");
}
