fn main() {
    println!("fib(n) = {}", fib(1));
    println!("fib(n) = {}", fib(2));
    println!("fib(n) = {}", fib(3));
    println!("fib(n) = {}", fib(4));
    println!("fib(n) = {}", fib(5));
    println!("fib(n) = {}", fib(6));
    println!("fib(n) = {}", fib(7));
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        return 1;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}
