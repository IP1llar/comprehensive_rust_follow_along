// The `match` keyword lets you match a value against one or more patterns.
// The comparisons are done from top to bottom and the first match wins.
// Match can be used as an expression - each arm must have the same type
// A variable in the pattern will create a binding that can be used within the match arm
// A match guard causes the arm to match only if the condition is true

#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q'                         => println!("Quitting"),
        'a' | 's' | 'w' | 'd'       => println!("Moving around"),
        '0'..='9'                   => println!("Number input"),
        key if key.is_lowercase()   => println!("Lowercase: {key}"),
        _                           => println!("Something else"),
    }
}
