fn main() {
    // Used exactly like if statements in other languages
    let x = 10;
    if x < 20 {
        println!("small");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
    // You can use if as an expression - the last expression of each block becomes the value of the
    // if expression
    let size = if x < 20 {"small"} else if x < 100 {"biggish"} else {"large"};
    println!("number size: {}", size);
}
