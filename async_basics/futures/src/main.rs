// `Future` is a trait, implemented by objects that represent an operation that may not be complete
// yet.
// A future can be polled, and `poll` returns a `Poll`
use std::pin::Pin;
use std::task::Context;

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
fn main() {
    println!("Hello, world!");
}
