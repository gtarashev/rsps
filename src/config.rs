use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use libc::STDIN_FILENO;
use crate::environment::Environment;

fn init_term() -> Termios {
    let termios = Termios::from_fd(STDIN_FILENO).unwrap();
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &mut new_termios).unwrap();
    return termios;
}

pub fn reset_term(env: &Environment) {
    tcsetattr(STDIN_FILENO, TCSANOW, &env.termios).unwrap();
}

pub fn initialise() -> Environment {
    let termios = init_term();
    Environment::new("> ".to_string(), termios)
}
