// Standard hash map with protection against HashDoS attacks
// `HashMap` is not defined in the prelude and must be brought into scope
use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We know about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");

    // This line will see if a book is in the hahmap and if not return an alternative value.
    let pc1 = page_counts
        .get("Harry Potter and the Sorcerer's Stone")
        .unwrap_or(&336);
    println!("pc1: {}", pc1);

    // This line will insert the alternative value in the hashmap if the book is not found.
    let pc2 = page_counts
        .entry("The Hunger Games".to_string())
        .or_insert(374);
    println!("pc2: {}", pc2);

    // Unlike `vec!`, there is no standard `hashmap!` 
    // - Since Rust 1.56, HashMap implements `From<[(K, V); N]>`, which allows us to  easily initialize
    // a hash map from a literal array:
    let page_counts_from_vec = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);
    println!("{page_counts_from_vec:#?}");

    // Alternatlively HashMap can be built from any `Iterator` which yields key-value tuples
    // Using references in collections can, of course, be done, but it can lead into complications
    // with the borrow checker

    // This type has several "method-specific" return types, such as
    // `std::collections::hash_map::Keys`. These types often appear in searches of the Rust docs
}
