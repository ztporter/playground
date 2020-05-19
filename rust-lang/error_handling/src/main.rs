use std::io::{self, Read};
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
