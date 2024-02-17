// Normal threads cannot borrow from their environment:
use std::thread;

/*
fn non_scoped_thread() {
    let s = String::from("Hello");
    let handle = thread::spawn(|| {
        println!("Length: {}", s.len());
    });
    handle.join();
}
*/

// However, you can use a `scoped thread` for this:
fn main() {
    let s = String::from("Hello");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    })
}
// The reason for that is that when the `thread::scope` function completes, all the threads are
// guaranteed to be joined, so they can return borrowed data
//
// Normal Rust borrowing rules apply: you can either borrow mutably by one thread, or immutably by
// any number of threads
