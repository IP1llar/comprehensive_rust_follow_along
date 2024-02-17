// Rust threads work similarly to threads in other languages:
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Count in handle thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
        return "This is the return from spawned thread"
    });

    let val = handle.join();

    println!("val: {val:?}");

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}

// Notice that the thread is stopped before it reaches 10 - the main thread is not waiting!
//
// Use `let handle = thread::spawn(...)` and later `handle.join()` to wait for the thread to finish
//
// Triggering a panic in the thread doesn't affect `main`
//
// You can use the `Result` to return value from `handle.join()` to get access to the panic
// payload.
// This is a good time to talk about `any` next
