fn main() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    // Below line panics because 'byte index 12 is not a character boundary; it is inside the
    // planet emoji
    // println!("{:?}", &sentence[12..13]);
    println!("{:?}", &sentence[11..15]);
    // Raw strings allow you to create a &str value with escapes disabled `r"\n" == "\\n"`
    // You can embed double-quotes by using an equal amount of # on either side of the quotes:
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}
