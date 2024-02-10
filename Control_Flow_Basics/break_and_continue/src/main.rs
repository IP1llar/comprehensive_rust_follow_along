fn main() {
    // If you want to exit any kind of loop early, use `break`.
    // If you want to immediately start the next iteration, use `continue`.
    let (mut a, mut b) = (100, 52);
    let result = loop {
        if a == b {
            break a;
        }
        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    };
    println!("{result}");

    // Both continue and break can optionally take a label argument which is used to break out of
    // nested loops:
    'outer: for x in 1..5 {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}
