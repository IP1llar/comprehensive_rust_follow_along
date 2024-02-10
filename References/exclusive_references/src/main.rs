// Exclusive references, also known as mutable references, allow changing the value they refer to.
// They have type `&mut T`
//
// Exclusive means that only this reference can be used to access the value.
// No other references (shared or exclusive) can exist at the same time, and the referenced value
// cannot be accessed while the exclusive reference exists.
fn main() {
    let mut point = (1,2);
    let x_coord = &mut point.0;
    // point.0 = 2; unallowed as point is 'borrowed' in x_coord
    *x_coord = 20;
    println!("point: {point:?}");
    point.0 = 5;
    println!("point: {point:?}");
}

// The following are different:
// let mut x_coord: &i32    - represents a shared reference which can be bound to different values
// let x_coord: &mut i32    - represents an exclusive reference to a mutable variable
