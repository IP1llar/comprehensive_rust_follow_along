// Let us see `Arc` and `Mutex` in action:
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));

    let v2 = Arc::clone(&v);
    let handle = thread::spawn(move || {
        let mut v2 = v2.lock().unwrap();
        v2.push(10);
    });

    {
        let mut v = v.lock().unwrap();
        v.push(1000);
    }
    handle.join().unwrap();
    println!("v: {v:?}");
}

// `v` is wrapped in both `Arc` and `Mutex`, because their concerns are orthogonal
// - Wrapping a `Mutex` in an `Arc` is a common pattern to share mutable state between threads
//
// `v: Arc<_>` needs to be cloned as `v2` before it can be moved into another thread. Note `move`
// was added to the lambda signature
//
// Blocks are introduced to narrow the scope of the `LockGuard` as much as possible
