/***        imports             ***/
use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;
use std::env;
use termios::Termios;

/***        structs             ***/
#[derive(Clone)]
pub struct Environment {
    pub ps1: String,
    pub previous_code: i32,
    pub previous_dir: PathBuf,
    pub history: Vec<String>,
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
    pub fn new(ps1: String, termios: Termios) -> Environment {
        Environment {
            ps1,
            previous_code: 0,
            previous_dir: env::current_dir().expect("couldn't set previous dir"),
            history: Vec::new(),
            termios,
        }
    }
}
