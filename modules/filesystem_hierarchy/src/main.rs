// Omitting the module content will tell Rust to look for it in another file:
mod garden;
// This tells rust that the `garden` module content is found at `src/garden.rs`
// Similarly, a `garden::vegetables` module could be found at `src/garden/vegetables.rs`
//
// The `crate` root is in:
// - `src/lib.rs` (for a library crate)
// - `src/main.rs` (for a binary crate)
//
// Modules defined in files can be documented too, using "inner foc comments"
// - These document the item that contains them - in this case, a module (see `src/garden.rs`)
//
fn main() {
    let x = garden::SeedPacket(2, 3);
    let mut y = garden::Garden(4);
    let _z = garden::sow(vec![x]);
    let _w = garden::harvest(&mut y);
    println!("Modules!");
}

// Before Rust 2018, modules needed to be located at `module/mod.rs` intsead `module.rs`, and this
// is still a working alternative editions after 2018
//
// The main reason to introduce `filename.rs` as alternative to `filename/mod.rs` was because many
// files named `mod.rs` can be hard to distinguish in IDEs
//
// Deeper nesting can use folders, even if the main module is a file:
// 
// src/
// |- main.rs
// |- top_module.rs
// |- top_module/
//      |- sub_module.rs
// 
// The place rust will look for modules can be changed with a compiler directive:
//
// ```
// #[path = "some/path.rs"]
// mod some_module;
// ```
//
// This is useful, for example, if you would like to place tests for a module in a file named
// `some_module_test.rs`, similar to the convention in Go.
