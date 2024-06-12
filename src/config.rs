/*          imports             */
use std::io::{stdin, stdout};
use termios::{Termios, tcsetattr};
use termios::{TCSANOW, ECHO, ICANON, ISIG, IEXTEN};
use termios::{VMIN, VTIME};

use libc::STDIN_FILENO;
use crate::environment::Environment;

/*          functions           */
fn init_term() -> Termios {
    let termios = Termios::from_fd(STDIN_FILENO).unwrap();
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ICANON | ECHO | ISIG | IEXTEN);
    new_termios.c_cc[VMIN] = 0;
    new_termios.c_cc[VTIME] = 1;
    tcsetattr(STDIN_FILENO, TCSANOW, &mut new_termios).unwrap();
    return termios;
}

pub fn reset_term(env: &Environment) {
    tcsetattr(STDIN_FILENO, TCSANOW, &env.termios).unwrap();
}

pub fn initialise() -> Environment {
    let termios = init_term();
    let stdin = stdin().lock();
    let stdout = stdout().lock();
    Environment::new("> ".to_string(), termios, stdin, stdout)
}
