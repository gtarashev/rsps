/*          imports             */
use std::io::Write;

/*          functions           */
pub fn clear_line<T: Write>(stdout: &mut T) {
    write!(stdout, "\r").unwrap();
    if let Err(e) = stdout.flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}

pub fn print_line<T: Write>(stdout: &mut T, ps1: &String, output: &String) {
    write!(stdout, "{}{}", ps1, output).unwrap();
    if let Err(e) = stdout.flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}
