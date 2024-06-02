use std::io;
use std::io::Read;

pub fn get_char() -> char {
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    reader.read_exact(&mut buffer).unwrap();

    return buffer[0] as char;
}
