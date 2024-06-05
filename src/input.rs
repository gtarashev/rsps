use std::io;
use std::io::Read;

pub fn std_get_char() -> char {
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    match reader.read_exact(&mut buffer) {
        Ok(()) => (),
        Err(_) => buffer[0] = b'\0',
    }

    return buffer[0] as char;
}

// some special characters, such as arrow keys are up to 3 bytes long, this means that reading a
// single byte and turning it into a character is not enough, the reading will most likely be done
// using this method, as opposed to `get_char`, but keeping other one just in case
pub fn std_read_into_buffer() -> (usize, [u8; 3]) {
    let mut reader = io::stdin();
    let mut buffer = [0;3];
    let size = reader.read(&mut buffer).unwrap();
    return (size, buffer);
}
