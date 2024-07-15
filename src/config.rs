/*          imports             */
use std::io::{Read, Write, stdin, stdout};
use termios::{Termios, tcsetattr};
use termios::{TCSANOW, ECHO, ICANON, ISIG, IEXTEN};

use crate::environment::Environment;
use crate::rsps_loop::shell_loop;

const STDIN_FILENO: i32 = 1;

/*          functions           */
fn init_term() -> Termios {
    let termios = Termios::from_fd(STDIN_FILENO).unwrap();
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ICANON | ECHO | ISIG | IEXTEN);
    tcsetattr(STDIN_FILENO, TCSANOW, &mut new_termios).unwrap();
    return termios;
}

fn reset_term<R, W>(env: &Environment<R, W>)
where R: Read,
      W: Write {
    tcsetattr(STDIN_FILENO, TCSANOW, &env.termios).unwrap();
}

pub fn initialise() {
    let termios = init_term();
    let stdin = stdin();
    let stdout = stdout();
    let mut env = Environment::new("> ".to_string(), termios, stdin, stdout);
    shell_loop(&mut env);
    reset_term(&env);
}
