/*          imports             */
use std::io::{StdinLock, Read};

/*          functions           */
pub fn std_read_into_buffer(stdin: &mut StdinLock<'static>, buffer: &mut [u8; 3]) -> usize {
    let size = stdin.read(buffer).unwrap();
    return size;
}
