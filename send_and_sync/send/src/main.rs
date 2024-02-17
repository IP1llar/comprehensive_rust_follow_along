// A type `T` is `Send` if it is safe to move a `T` value to another thread
//
// The effect of moving ownership to another thread is that destructors will run in that thread. So
// the question is when you can allocate a value in one thread and deallocate it in another
//
// As an example, a connection to the SQLite library must only be accessed from a single thread
//
fn main() {
    println!("Hello, world!");
}
