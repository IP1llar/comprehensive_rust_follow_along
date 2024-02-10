fn main() {
    // while
    println!("While loop:");
    let mut x = 200;
    while x >= 10 {
        println!("-- x = {x}");
        x = x/2;
    }
    println!("Final x: {x}");
    println!("");

    // for
    println!("For loop:");
    for x in 1..5 {
        println!("x: {x}");
    }
    println!("");

    // loop - loops forever, until a break
    println!("loop:");
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 10 {
            println!("Break loop");
            break;
        }
    }
    println!("Loop end:{i}");
}
