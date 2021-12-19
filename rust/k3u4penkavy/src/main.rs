use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let mut line = String::from("");
    // std::io::stdin().read_line(&mut line);
    let mut f = File::open("controlInput.txt").expect("Unable to open");
    let mut buf_reader = BufReader::new(f);
    buf_reader.read_to_string(&mut line);
    println!("Text: {}, bytes: {:?}", line, line.as_bytes());
}
