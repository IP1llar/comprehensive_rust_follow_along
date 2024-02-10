// `Box` is an owned pointer to data on the heap:
fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);

    list_demo();
}

// Stack        Heap
//
// five
// ------         ---  
// | ---|-------->|5|
// ------         ---
//
// `Box<T>` implements `Deref<Target = T>`, which means that you can call methods from `T` directly
// on a `Box<T>`
//
// Recursive data types or data types with dynamic sizes need to use a `Box`:
#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

fn list_demo() {
    let list: List<i32> = 
        List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}


// Stack
//
// list                     Heap
//  --------------          --------------    ----------
// |Element|1| --|-------->|Element|2| --|-->|Nil|//|//|
// --------------          --------------    ----------
//
//
// `Box` is like `std::unique_ptr` in C++, except that it's guaranteed to be not null
//
// A `Box` can be useful when you:
// - have a type whose size that can;t be known at compile time, but the compiler wants to know an
// exact size
// - want to transfer ownership of a large amount of data - to avoid copying large amounts of data
// on the stack, instead store the data on the heap in a `Box` so only the pointer is moved
//
// If `Box` was not used and we attempted to embed a `List` directly into the `List`, the compiler
// would not compute a fixed size of the struct in memory (`List` would be of infinite size)
//
// `Box` solves this problem as it has the same size as a regular pointer and just points at the
// next element of the `List` in the heap
//
// Removing the `Box` in the List definition shows the compiler error "Recursive with indirection",
// which is a hint that you might want to use a `Box` or reference of some kind, instead of storing
// a value directly
//
//
// Niche optimization:
// A `Box` cannot be empty, so the pointer is always valid and non-`null`. This allows the compiler
// to optimize the memory layout
//
// list                     Heap
//  --------------          -------------
// |Element|1| --|-------->|Element|2|//|
// --------------          -------------
//
