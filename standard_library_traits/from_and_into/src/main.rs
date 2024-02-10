// Types implement `From` and `Into` to facilitate type conversions:
fn main() {
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123_i16);
    println!("{s}, {addr}, {one}, {bigger}");

    // `Into` is automatically implemented when `From` is implemented
    let s2: String = "hello".into();
    let addr2: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one2: i16 = true.into();
    let bigger2: i32 = 123_i16.into();
    println!("{s2}, {addr2}, {one2}, {bigger2}");

    // That's why it is common to only implement `From`, as your type will get `Into`
    // implementation too
    //
    // When declaring a function argument input type like "anything that can be converted into a
    // `String`", you should use `Into`. You function will accept types that implement `From` and
    // those that only implement `Into`
}
