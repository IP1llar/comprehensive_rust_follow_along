// Destructuring is a way of extracting data from a data structure by writing a pattern that is
// matched up to the data structure, binding variables to subcomponents of the data structure.

#[rustfmt::skip]
fn main() {
    describe_point((1, 0));

    let triple = [2, -2, 4];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("First is 1 and the rest were ignored"),
        [.., 4] => println!("Last is 4 and the rest are ignored"),
        _ => println!("All elements were ignored"),
    }
}

fn describe_point(point: (i32, i32)) {
    match point {
        (0, 0) => println!("on origin"),
        (0, _) => println!("on Y axis"),
        (_, 0) => println!("on X axis"),
        (x, _) if x < 0 => println!("left of Y axis"),
        (_, y) if y < 0 => println!("below x axis"),
        _ => println!("first quadrant"),

    }
}
