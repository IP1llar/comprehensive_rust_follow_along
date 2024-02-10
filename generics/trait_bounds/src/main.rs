// When working with generics, you often want to require the types to implement some trait, so that
// you can call this trait's methods
//
// You can do this with `T: Trait` or `impl Trait`
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// struct NotClonable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);

    // let bar = NotClonable;
    // let bar_copy = duplicate(bar);
    println!("{pair:?}");
}

// When multiple are necessary, use `+` to join them
//
// A `where` clause can be used to declutter the function signature if you have many parameters
// - It has additional features making it more powerful
// - - The extra feature is that the type on the left of `:` can be arbitrary, like `Option<T>`

fn _duplicate<T>(a: T) -> (T, T)
where 
    T: Clone,
{
    (a.clone(), a.clone())
}

// Rust does not yet support specialization
// e.g. given the original `duplicate`, it is invalid to add a specialized `duplicated(a: u32)`
