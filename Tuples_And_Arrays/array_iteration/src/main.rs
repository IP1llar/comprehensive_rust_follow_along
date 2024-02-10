// The `for` statement supports for iterating over arrays (but not tuples).
// This functionality uses the `IntoIterator` trait


fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for  i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}
