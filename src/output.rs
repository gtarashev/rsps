/*          imports             */
use std::io::Write;
use std::fmt::Display;

/*          functions           */
pub fn clear_line<T: Write>(stdout: &mut T) {
    write!(stdout, "\r\x1b[K").unwrap();
    if let Err(e) = stdout.flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}

pub fn print_line<T, D>(stdout: &mut T, ps1: &D, output: &D) 
where T: Write,
D: Display {
    write!(stdout, "{}{}", ps1, output).unwrap();
    if let Err(e) = stdout.flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}
