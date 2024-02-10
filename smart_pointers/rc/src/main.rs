// `Rc` is a reference-counted shared pointer. Use this when you need to refer to the same data
// from multiple places:
use std::rc::Rc;

fn main() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);

    println!("a: {a}");
    println!("a count: {}", Rc::strong_count(&a));
    println!("b: {b}");
    println!("b count: {}", Rc::strong_count(&b));
}

// `Arc` and `Mutex` are useful for multi-threaded context
// You can downgrade a shared pointer into a `Weak` pointer to create cycles that will get dropped
//
// `Rc`'s count ensures that its contained value is valid for as long as there are references
//
// `Rc` in Rust is like `std::shared_ptr` in C++
//
// `Rc::clone` is cheap: it creates a pointer to the same allocation and increases the
// reference count. Does not make a deep clone and can generally be ignored when looking for
// performance issues in code
//
// `make_mut` actually clones the inner value if necessary ("clone-on-write") and returns a mutable
// reference
//
// Use `Rc::strong_count` to check the reference count
//
// `Rc::downgrade` gives you a weakly reference-counted object to create cycles that will be
// dropped properly (likely in combination with `RefCell`)
