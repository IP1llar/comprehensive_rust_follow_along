fn main() {
    // Rust infers type from usage - this is not a dynamic type that can hold any data
    let x = 10;
    //  ^u32
    let y = 20;
    //  ^i8

    takes_u32(x);
    takes_i8(y);

    // takes_u32(y);
    // ^panics as argument is incorrect type
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}
