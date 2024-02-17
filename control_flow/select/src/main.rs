// A select operation waits until any of a set of futures is ready, and responds to that future's
// result. In JS, this is imilar to `Promise.race`. In Python, it compares to
// `asyncio.wait(task_set, return_when=asyncio.FIRST_COMPLETED)`.
//
// Similar to a match statement, the body of `select!` has a number of arms, each of the form
// `pattern = future => statement`. When a `future` is ready, its return value is destructured by
// the `pattern`. The `statement` is the run with the resulting variables. The `statement` result
// becomes the result of the `select!` macro.
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq)]
enum Animal {
    Cat { name: String },
    Dog { name: String },
}

async fn first_animal_to_finish_race(
    mut cat_rcv: Receiver<String>,
    mut dog_rcv: Receiver<String>,
) -> Option<Animal> {
    tokio::select! {
        cat_name = cat_rcv.recv() => Some(Animal::Cat { name: cat_name? }),
        dog_name = dog_rcv.recv() => Some(Animal::Dog { name: dog_name? })
    }
}

#[tokio::main]
async fn main() {
    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);
    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        cat_sender.send(String::from("Felix")).await.expect("Failed to send cat.");
    });
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        dog_sender.send(String::from("Rex")).await.expect("Failed to send dog.");
    });

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver)
        .await
        .expect("Failed to receive winner");

    println!("Winner is {winner:?}");
}
// You can use `oneshot` channels in this example as the channels are supposed to receive only one
// `send`.
//
// Note that `select!` drops unmatched branches, which cancels their futures. It is easiest to use
// when every execution of `select!` creates new futures.
// - An alternative is to pass `&mut future` instead of the future itself, but this can lead to
// issues (see pinning)
