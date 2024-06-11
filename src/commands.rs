/***        imports             ***/
use crate::environment::Environment;
use std::collections::VecDeque;
use std::env;
use std::path::Path;
use std::process::{Command, Stdio, Child};

/***        functions           ***/
pub fn process_command(env: &mut Environment, input: &str) -> Option<i8> {
    if input == "\n".to_string() {
        env.previous_code = 0;
        return None;
    }

    env.history.push_back(input.trim().to_string());
    // when initialised, first element is empty, we want to remove that element
    // so the history is cleaner
    if env.history[0] == "".to_string() { 
        env.history.pop_front();
    }

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
            "history" => {
                let mut history_copy = env.history.clone();
                history_copy.make_contiguous();
                for command in history_copy {
                    println!("{}", command);
                }
            },
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
