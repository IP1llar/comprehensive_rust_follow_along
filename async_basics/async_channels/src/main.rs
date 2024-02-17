// Several crates have support for asynchronous channels. For instance `tokio`:
use tokio::sync::mpsc::{self, Receiver};

async fn ping_handler(mut input: Receiver<()>) {
    let mut count: usize = 0;
    
    while let Some(_) = input.recv().await {
        count += 1;
        println!("Received {count} pings so far.");
    }

    println!("ping_handler complete");
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(32);
    let ping_handler_task = tokio::spawn(ping_handler(receiver));
    for i in 0..10 {
        sender.send(()).await.expect("Failed to send ping.");
        println!("Sent {} pings so far.", i + 1);
    }
    
    drop(sender);
    ping_handler_task.await.expect("Something went wrong in ping handler task.");
}

// Changing the channel size to `3` only allows up to 3 pings queued at a time
//
// Overall, the interface is similar to the `sync` channels
//
// Removing the `std::mem::drop` call causes the program to not stop, as the receiver is still
// listening
//
// The Flume crate has channels that implement both `sync` and `async` `send` and `recv`. This can
// be convenient for complex applications with both IO and heavy CPU processing tasks
//
// What makes working with `async` channels preferable is the ability to combine them with other
// `future`s to combine them and create complex control flow
//
