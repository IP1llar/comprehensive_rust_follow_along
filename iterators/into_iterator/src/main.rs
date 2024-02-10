// The `Iterator` trait tells you how to iterate once you have created an iterator
// - The related trait `IntoIterator` defines how to create an iterator for a type
// - It is automatically used by the `for` loop

struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl IntoIterator for Grid { 
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter { grid: self, i: 0, j: 0 }
    }
}


struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn main() {
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    println!("Non-ref run through");
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }
    ref_demo();
}

// Every implementation of `IntoIterator` must declare two types:
// - `Item`: the type to iterate over, such as `i8`,
// - `IntoIter`: the `Iterator` type returned by the `into_iter` method
//
// Note that `IntoIter` and `Item` are linked: the iterator must have the same `Item` type, which
// means that it returns `Option<Item>`
//
// The example iterates over all combinations of x and y coordinates
//
// If you try to iterate over the grid twice in `main`, it fails because `IntoIterator::into_iter`
// takes ownership of `self`
//
// This can be fixed by implementing `IntoIterator` for `&Grid` and storing a reference to the
// `Grid` in `GridIter` or a new struct`RefGridIter` as seen below
//
// The same problem can occur for standard library types: `for e in some_vector` will take
// ownership of `some_vector` and iterate over owned elements from that vector
// - Use `for e in &some_vector` instead, to iterate over references to elements of `some_vector`
//
fn ref_demo() {
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    println!("First run through");
    for (x, y) in &grid {
        println!("point = {x}, {y}");
    }
    println!("Second run through");
    for (x, y) in &grid {
        println!("point = {x}, {y}");
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (u32, u32);
    type IntoIter = RefGridIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RefGridIter {
            grid: self, i: 0, j: 0
        }
    }
}

struct RefGridIter<'a> {
    grid: &'a Grid,
    i: usize,
    j: usize,
}

impl Iterator for RefGridIter<'_> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}
