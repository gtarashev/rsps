/***        imports             ***/
use std::{
    collections::VecDeque,
    env::current_dir,
    fmt::{Display, Formatter, Result},
    io::{Read, Write},
    path::PathBuf,
};

use termios::Termios;

/***        structs             ***/
pub struct Environment<R, W>
where
    R: Read,
    W: Write,
{
    pub ps1: String,
    pub previous_code: i32,
    pub previous_dir: PathBuf,
    pub history: VecDeque<String>,
    pub stdin_handle: R,
    pub stdout_handle: W,
    pub termios: Termios,
}

/***        functions           ***/
impl<R, W> Display for Environment<R, W>
where
    R: Read,
    W: Write,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write! {f, "Environment: \n\tPS1: {}\n\tprevious_code: {}\n\tprevious_dir: {:?}", self.ps1, self.previous_code, self.previous_dir}
    }
}

impl<R, W> Environment<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(
        ps1: String,
        termios: Termios,
        stdin_handle: R,
        stdout_handle: W,
    ) -> Environment<R, W> {
        Environment {
            ps1,
            previous_code: 0,
            previous_dir: current_dir().expect("couldn't set previous dir"),
            history: VecDeque::new(),
            termios,
            stdin_handle,
            stdout_handle,
        }
    }
}
