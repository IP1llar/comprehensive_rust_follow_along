// `String` is the standard heap-allocated growable UTF-8 string buffer:
fn main() {
    // `String::new` returns a new empty string, use `String::with_capacity` when you know how much
    // data you want to push to the string
    // `String::len` returns the size of the `String` in bytes (which can be different from its
    // length in characters)
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    // `String::chars` returns an iterator over the actual characters. Note that a `char` can be
    // different from what a human will consider a "character" due to grapheme clusters
    println!("s3: len = {}, number of chars = {}", s3.len(), s3.chars().count());
}
// When people refer to strings they could either be talking about`&str` or `String`
// When a type implements `Deref<Target = T>`, the compiler will let you transparently call methods
// from `T`
// - We haven't discussed the `Deref` trait yet
// - `String` implements `Deref<Target = str>` which transparently gives it access to `str`'s
// methods
//
// `String` is implemented as a wrapper around a vector of bytes, many of the operations you see
// supported on vectors are also supported on `String`, but with some extra guarentees
//
// Compare the different ways to index a `String`:
// - To a character by using `s3.chars().nth(u).unwrap()` where `i` is in-bound, out-of-bounds
// - To a substring by using `s3[0..4]`, where that slice is on character boundaries or not
