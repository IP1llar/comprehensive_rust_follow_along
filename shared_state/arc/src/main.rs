// `Arc<T>` allows shared read-only access via `Arc::clone`:
use std::sync::Arc;
use std::thread;

fn main() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..=5 {
        let v = Arc::clone(&v);
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");
}

// `Arc` stands for "Atomic Reference Counted", a thread safe version of `Rc` that uses atomic
// operations
//
// `Arc<T>` implements `Clone` whether or not `T` does. It implements `Send` and `Sync` if and only
// if `T` implements them both
//
// `Arc::clone()` has the cost of atomic operations that get executed, but after that the use of
// the `T` is free
//
// Beware of reference cycles, `Arc` does not use a garbage collector to detect them
// - `std::sync::Weak` can help
