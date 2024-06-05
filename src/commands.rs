/***        imports             ***/
use crate::environment::Environment;
use std::collections::VecDeque;
use std::env;
use std::path::Path;
use std::process::{Command, Stdio, Child};

/***            methods             ***/
pub fn process_command(env: &mut Environment, input: &str) -> Option<i8> {
    let mut command_list = input.trim().split("|").peekable();
    let mut previous_command = None;
    
    while let Some(x) = command_list.next() {
        let mut arguments: VecDeque<&str> = x.trim().split_whitespace().collect();
        
        match arguments.pop_front().unwrap_or("") {
            "cd" => {
                env.previous_dir = env::current_dir().expect("couldn't get current dir");
                let mut new_dir = "/home/jojito";
                if let Some(x) = arguments.pop_front() {
                    new_dir = x;
                }

                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                    env.previous_code = 1;
                } else {
                    env.previous_code = 0;
                }
            },
            "history" => println!("{:?}", env.history),
            "previous" => println!("{}", env.previous_code),
            "exit" => return Some(1),
            command => {
                if command == "".to_string() {
                    return None;
                }

                // map stdin
                let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));
                // set stdout
                let stdout = if command_list.peek().is_some() {
                    Stdio::piped()
                } else {
                    Stdio::inherit()
                };
        
                let child = Command::new(command).args(arguments).stdin(stdin).stdout(stdout).spawn();

                match child {
                    Ok(output) => previous_command = Some(output),
                    Err(e) => {
                        previous_command = None;
                        eprintln!("Error: {}", e);
                    }
                }
            },
        }
    }
    
    if let Some(mut final_command) = previous_command {
        match final_command.wait().expect("something").code() {
            Some(code) => env.previous_code = code,
            None => {
                env.previous_code = 999;
            },
        }
    }
    return None;
}
