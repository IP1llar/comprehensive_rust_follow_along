// Modules are a privacy boundary:
// - Module items are private by default (hides implementation details)
// - Parent and sibling items are always visible
// - In other words, if an item is visible in module `foo`, it's visible in all the descendants of
// `foo`
mod outer {
    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

fn main() {
    outer::public();
}
// Use the `pub` keyword to make modules public
//
// Additionally, there are advanced `pub(...)` specifiers to restrict the scope of public
// visibility
// - More details in the Rust Reference
// - Configuring `pub(crate)` visibility to a specific path
// - Less commonly, you can give visibility to a specific path
// - In any case, visibility must be granted to an ancestor module (and all of its descendants)
