// Rust has support for custom structs
// Structs work like in C++ or C:
// - No typedef is needed to define a type
// - There is no inheritance between structs
//
// There are different types of structs:
// - Zero-sized structs (e.g. `struct Foo;`) might be used when implementing a trait on some type
// but don't have any data that you want to store in the value field itself
// - Tuple structs are used when the field names are not important
//
// Structs can be created using shorthand
// The syntax `..avery` allows us to copy the majority of the fields from the old struct without
// having to explicitly type it all out. It must always be the last element.

struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let mut peter = Person {name: String::from("Peter"), age: 27};
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person {name, age};
    describe(&avery);

    let jackie = Person { name: String::from("Jackie"), ..avery};
    describe(&jackie);
}
