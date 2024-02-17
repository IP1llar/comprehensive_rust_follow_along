// Tokio provides:
// - A multi-threaded runtime for executing asynchronous code.
// - An asynchronous version of the standard library
// - A large ecosystem of libraries

use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    //count_to(10).await;
    tokio::spawn(count_to(10));
    //let _ = tokio::spawn(count_to(10)).await;

    for i in 1..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

// With the `tokio::main` macro we can now make `main` async
// The `spawn` function creates a new, concurrent "task"
// Note: `spawn` takes a `Future`, you don't call `.await` on `count_to`
//
// Why does `count_to` not (usually) get to 10? This is an example of async cancellation.
// `tokio::spawn` returns a handle which can be awaited to wait until it finishes.
//
// Try `count_to(10).await` instead of spawning
//
//
