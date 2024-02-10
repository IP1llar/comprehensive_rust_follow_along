// The `enum` keyword allows for the creation of a type which has a few different variants:

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

// PlayerMove is a type with three variants
// In addition to the payloads, Rust will store a discriminant so that it knows at runtime which
// variant is a `PlayerMove` value
#[derive(Debug)]
enum PlayerMove {
    Pass,                       // Simple variant
    Run(Direction),             // Tuple variant
    Teleport {x: u32, y: u32},  // Struct variant
}

fn main() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);

    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}

// Rust uses minimal space to store the discriminant
// - If necessary, it stores an intefer of the smallest required size
// - If the allowed variant values do not cover all bit patterns, it will use invalid bit patterns
// to encode the discriminant (the "niche optimization"). For example, `Option<&u8>` stores either
// a pointer to an integer or `NULL` for the`None` variant.
// - The discriminant can be controlled if needed (e.g. for compatibility with C):
#[repr(u32)]
enum Bar {
    A, // 0
    B = 10000,
    C, // 10001
}

// Rust has several optimizations it can employ to make enums take up less space
// - Null pointer optimization: For some types, Rust guarentees that `size_of::<T>()` equals
// `size_of::<Option<T>>`
