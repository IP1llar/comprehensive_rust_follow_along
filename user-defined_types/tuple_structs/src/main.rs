// If field names are unimportant, you can use a tuple struct
struct Point(i32, i32);

// This is often used for single-field wrappers (called newtypes)
struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA");
}

fn set_thruster_force(force: Newtons) {
    todo!("Get yourself a rocket first");
}

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}
