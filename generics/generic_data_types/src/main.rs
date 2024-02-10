// You can use generics to abstract over the concrete fied type:

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// `T` is specified twice because:
// - It is a generic implementation for generic type - they are
// independently generic
// - It means these methods are defined for any `T`
// - It is possible to write `impl Point<u32> { .. }`
//  - `Point` is still generic and you can use `Point<f64>`, but methods in this block will only be
//  available for `Point<u32>`
impl<T, U> Point<T, U> {
    fn coords(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }

    // fn set_x(&mut self, x: T)
}

fn main() {
    let integer = Point { x: 5, y: 10.0 };
    let float = Point { x: 1.0, y: 4 };
    println!("{integer:?} and {float:?}");
    println!("coords: {:?}", integer.coords());
}
