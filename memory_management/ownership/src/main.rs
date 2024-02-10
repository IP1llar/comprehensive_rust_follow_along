// All variable bindings can have a scope where they are valid and it is an error to use a variable
// outside its scope:
struct Point(i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    println!("y: {}", p.1);
}

// We say that the variable owns the value
// - Every Rust value has precisely one owner at all times
//
// At the end of the scope, the variable is dropped and the data is freed
// - A destructor can run here to free up resources
