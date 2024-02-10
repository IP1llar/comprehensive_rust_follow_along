// Like tuples, structs and enums can also be destructured by matching:
struct Foo {
    x: (u32, u32),
    y: u32,
}

// Patterns can also be used to bind variables to parts of your values. This is how you inspect the
// structure of your types
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

#[rustfmt::skip]
fn main() {
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y }    => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }      => println!("y = 2, x = {i:?}"),
        Foo { y, .. }           => println!("y = {y}, other fields were ignored"),
    }

    let n = 100;
    let result = divide_in_two(n);
    let mut count = 0;
    loop {
    count += 1;
    if count > 9 {
            break;
        }
    match &result {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    };
    } 
}
