// Sometimes you want to make a copy of a value
// - The `Clone` trait accomplishes this

#[derive(Default, Debug)]
struct Backends {
    hostnames: Vec<String>,
    weights: Vec<f64>,
}

impl Backends {
    fn set_hostnames(&mut self, hostnames: &Vec<String>) {
        self.hostnames = hostnames.clone();
        self.weights = hostnames.iter().map(|_| 1.0).collect();
    }
}

fn main() {
    let mut backends = Backends {hostnames: Vec::new(), weights: Vec::new()};
    println!("backends before: {:?}", backends);
    backends.set_hostnames(&vec![String::from("Hello"), String::from("World")]);
    println!("backends after: {:?}", backends);
}

// It's common to "clone your way out" of problems with the borrow checker, and return later to try
// to optimize those clones away
