// Static and constant variables are two different ways to create globally-scoped values that
// cannot be moved or reallocated during the execution of the program.

// Constant variables are evaluated at compile time and their values are inlined wherever they are
// used:
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

// Static variables will be live during the whole execution of the program, and therefor will not
// move
static BANNER: &str = "Welcome to RustOS 3.14";
// These are not inlined upon use and have an actual associated memory location
// - Useful for unsafe and embedded code, and the variable lives through the entirety of the
// program execution.
// - `static` variables are accessible from any thread so must be `Sync` - interior mutability is
// possible through a`Mutex`, atomix or similar

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");

    println!("{BANNER}");
}

// Property                                         Static                              Constant
// Has an address in memory                         Yes                                 No (inlined)
// Lives for the entire duration of the program     Yes                                 No
// Can be mutable                                   Yes (unsafe)                        No
// Evaluated at compile time                        Yes (initialised at compile time)   Yes
// Inlined wherever it is used                      No                                  Yes
