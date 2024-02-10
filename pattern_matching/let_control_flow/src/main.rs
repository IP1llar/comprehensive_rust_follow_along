// Rust has a few control flow constructs which differ from other languages - they are used for
// pattern matching
// - `if let` expressions
// - `while let` expressions
// - `match` expressions

// `if let` expression lets youy execute different code depending on whether a value matches a
// pattern:

fn sleep_for(secs: f32) {
    let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
        dur
    } else {
        std::time::Duration::from_millis(500)
    };
    std::thread::sleep(dur);
    println!("slept for {:?}", dur);
}

// `let else` is most common - the `else` case must diverge (`return`, `break`, or panic)
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("got None"));
    };

    let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
        first_byte_char
    } else {
        return Err(String::from("got empty string"));
    };

    if let Some(digit) = first_byte_char.to_digit(16) {
        Ok(digit)
    } else {
        Err(String::from("not a hex digit"))
    }
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);

    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));

    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        println!("character: {c}");
    }
}
