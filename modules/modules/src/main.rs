// We have seen how `impl` blocks let us namespace functions to a type
//
// Similarly, `mod` lets us namespace types and functions:
mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

mod bar {
    pub fn do_somethig() {
        println!("In the bar module");
    }
}

fn main() {
    foo::do_something();
    bar::do_somethig();
}

// Packages provide functionality and include a `Cargo.toml` file that describes how to build a
// bundle of 1+ crates
//
// Crates are a tree of modules, where a binary crate creates an executable and a library crate
// compiles to a library
//
// Modules define organization and scope
