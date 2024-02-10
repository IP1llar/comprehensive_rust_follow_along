// Instead of transferring ownership when calling a function, you can let a function borrow the
// value:
#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    let p = Point(p1.0 + p2.0, p1.1 + p2.1);
    println!("&p.0: {:p}", &p.0);
    p
}

fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("&p3.0: {:p}", &p3.0);
    println!("{p1:?} + {p2:?} = {p3:?}");
}

// The `add` function borrows two points and returns a new point
// The caller retains ownership of the inputs
//
// The return from `add` is cheap because the compiler can eliminate the copy operation
// - In the "DEBUG" optimization level, the addresses should change, while they stay the same when
// changing to the "RELEASE" setting
//
// The Rust compiler can do return value optimization (RVO)
//
// In C++, copy elision has to be defined in the language specification because constructors can
// have side effects
// - In Rust, this is not an issue at all. If RVO did not happen, Rust will alwsays perform a
// simple and efficient `memcpy`
