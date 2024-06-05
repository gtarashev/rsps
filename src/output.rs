/*          imports             */
use std::io::{stdout, Write};

/*          functions           */
pub fn display_ps1(ps1: &String) {
    print!("{}", ps1);
    if let Err(e) = stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}
