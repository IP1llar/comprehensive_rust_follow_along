// Values which implement`Drop` can specify code to run when they go out of scope:
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    println!("Exiting main");

}

// Note that `std::mem::drop` is not the same as `std::ops::Drop::drop`
// Values are automatically dropped when they of out of scope
// When a value is dropped, if it implements `std::ops::Drop` then its `Drop::drop` implementation
// will be called
// All its fields will then be dropped too, whether or not it implements `Drop`
// `std::mem::drop` is just an empty function that takes any value
// - The significance is that it takes ownership of the value, so at the end of its scope it gets
// dropped
// - This makes it a convenient way to explicitly drop values earlier than they would otherwise of
// out of scope - This can be useful for objects that do some work on `drop`: releasing locks,
// closing files, etc...
