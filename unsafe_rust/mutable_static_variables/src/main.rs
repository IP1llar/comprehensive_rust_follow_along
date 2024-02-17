// It is safe to read an immutable static variable:
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("HELLO_WORLD: {HELLO_WORLD}");

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
    
    add_to_counter(42);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

// However, since data races can occur, it is unsafe to read and write mutable static variables:
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// The program here is safe because it is single-threaded. However, the Rust compiler is
// conservative and will assume the worst. Try removing the `unsafe` and see how the compiler
// explains that it is undefined behavior to mutate a static from multiple threads
//
// Using a mutable static is generally a bad idea, but there are some cases where it might make
// sense in low-level `no_std` code, such as implementing a heap allocator or working with some C
// APIs
// 
