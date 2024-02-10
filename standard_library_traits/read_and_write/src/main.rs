// Using `Read` and `BufRead`, you can abstract over `u8` sources:
use std::io::{BufRead, BufReader, Read, Result, Write};

fn count_lines<R: Read>(reader:R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

fn main() -> Result<()> {
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("lines in slice: {}", count_lines(slice));

    let file = std::fs::File::open(std::env::current_exe()?)?;
    match std::env::current_exe() {
        Ok(v) => println!("{:?}",v),
        Err(e) => println!("{:?}",e),
    };
    println!("lines in file: {}", count_lines(file));

    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {:?}", buffer);
    Ok(())
}
