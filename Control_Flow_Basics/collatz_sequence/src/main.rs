fn main() {
    println!("Recursive length is: {}", collatz_length_recursive(11));
    println!("Iterative length is: {}", collatz_length_iterative(11));
}

fn collatz_length_recursive(n: i32) -> u32 {
    println!("Current is {n}");
    if n == 1 {
        return 1;
    } else if n%2 == 0 {
        return collatz_length_recursive(n/2) + 1;
    } else {
        return collatz_length_recursive(3*n + 1) + 1;
    }
}

fn collatz_length_iterative(mut n: i32) -> u32 {
    let mut length = 0;
    loop {
        println!("Current is {n}");
        length = length + 1;
        if n == 1 {
            break;
        }
        n = if n%2 == 0 {n/2} else {3*n + 1};
    }
    return length;
}
