// Operator overloading is implemented via traits in `std::ops`:
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
}

// You could implement `Add` for `&Point`
// - `Add:add` consumes self
// - If the type `T` for which you are overloading the operator is not `Copy`, you should consider
// overloading the operator for `&T` as well - this avoids unnecessary cloning on the call site
//
// You could implement `Add` for two different types, e.g. `impl Add<(i32, i32)> for Point` would
// add a tuple to a `Point`
