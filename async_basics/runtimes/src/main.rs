// A runtime provides support for performing operations asynchronously (a reactor) and is
// responsible for executing futures (an executor). Rust does not have a "built-in" runtime, but
// several options are available:
// - `Tokio`: performant, with a well-developed ecosystem of functionality like `Hyper` for HTTP or
// `Tonic` for gRPC
// - `async-std`: aims to be a "std for async", and includes a basic runtime in `async::task`
// - `smol`: simple and lightweight
//
// Several larger applications have their own runtimes. For example, Fuchsia already has one
//
// Futures are "inert" in that they do not do anything (not even start an I/O operation) unless
// there is an executor polling them. This differs from JS Promises, for example, which will run to
// completion even if they are never used.
fn main() {
    println!("Hello, world!");
}
