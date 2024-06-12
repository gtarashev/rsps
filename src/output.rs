/*          imports             */
use std::io::{Write, StdoutLock};

/*          functions           */
pub fn clear_line(stdout: &mut StdoutLock<'static>) {
    write!(stdout, "\x1b[{}D\x1b[K", 9999999).unwrap();
    if let Err(e) = stdout.flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}

pub fn print_line(stdout: &mut StdoutLock<'static>, ps1: &String, output: &String) {
    write!(stdout, "{}{}", ps1, output).unwrap();
    if let Err(e) = stdout.flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}
