/*          imports             */
use std::io::Read;

/*          functions           */
pub fn read_into_buffer<T: Read>(stdin: &mut T, buffer: &mut [u8; 3]) -> usize {
    let size = stdin.read(buffer).unwrap();
    return size;
}
