use std::io;
use std::io::Read;

pub fn get_char() -> char {
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    match reader.read_exact(&mut buffer) {
        Ok(()) => (),
        Err(_) => buffer[0] = b'\0',
    }

    return buffer[0] as char;
}
