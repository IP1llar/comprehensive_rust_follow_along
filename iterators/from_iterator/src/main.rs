// `FromIterator` lets you build a collection from an `Iterator`
fn main() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|p| p * p).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");

    ex_type_inference();
}
// Iterator implements the following:
// fn collect<B>(self) -> B
// where
//     B: FromIterator<Self::Item>,
//     Self: Sized
//
// There are two ways to specify `B` for this method:
// - With the "turbofish": `some_iterator.collect::<COLLECTION_TYPE>()`, as shown. The `_`
// shorthand used above lets Rust infer the type of the `Vec` elements
// - With type inference: `let prime_squares: Vec<_> = some_iterator.collect()` as shown below
fn ex_type_inference() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares: Vec<_> = primes.into_iter().map(|p| p * p).collect();
    println!("prime_squares inferred: {prime_squares:?}");
}

// There are basic implementations of `FromIterator` for `Vec`, `HashMap`, etc.
// There are also more specialized implementations which let you do cool things like convert an
// `Iterator<Item = Result<V, E>>` into a `Result<Vec<V>, E>`
