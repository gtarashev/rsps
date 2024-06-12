/***        imports             ***/
use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;
use std::env;
use std::io::{StdinLock, StdoutLock};
use std::collections::VecDeque;
use termios::Termios;

/***        structs             ***/
pub struct Environment {
    pub ps1: String,
    pub previous_code: i32,
    pub previous_dir: PathBuf,
    pub history: VecDeque<String>,
    pub stdin_handle: StdinLock<'static>,
    pub stdout_handle: StdoutLock<'static>,
    pub termios: Termios,
}

/***        functions           ***/
impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>)
        -> Result
    {
        write! {f, "Environment: \n\tPS1: {}\n\tprevious_code: {}\n\tprevious_dir: {:?}", self.ps1, self.previous_code, self.previous_dir}
    }
}

impl Environment {
    pub fn new(ps1: String, termios: Termios, stdin_handle: StdinLock<'static>, stdout_handle: StdoutLock<'static>) -> Environment {
        Environment {
            ps1,
            previous_code: 0,
            previous_dir: env::current_dir().expect("couldn't set previous dir"),
            history: VecDeque::new(),
            termios,
            stdin_handle,
            stdout_handle,
        }
    }
}
