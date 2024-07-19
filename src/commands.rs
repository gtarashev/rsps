/***        imports             ***/
use crate::environment::Environment;
use crate::lexer::Lexer;
use std::{
    env,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

pub fn process_command<R, W>(env: &mut Environment<R, W>, input: &str) -> Option<i8>
where
    R: Read,
    W: Write,
{
    if input == "\n".to_string() {
        env.previous_code = 0;
        return None;
    }

    env.history.push_back(input.trim().to_string());
    if env.history[0] == "".to_string() {
        env.history.pop_front();
    }

    let command = input.chars().collect::<Vec<_>>();
    let mut command_list: Vec<Vec<String>> = vec![];
    let mut cl_count = command_list.len();
    let mut previous_command = None;
    let lexer = Lexer::new(&command);
    for i in lexer.into_iter() {
        let i = String::from_iter(i);
        if cl_count == command_list.len() {
            command_list.push(vec![i]);
            continue;
        }

        if i == "|" {
            cl_count += 1;
            continue;
        }

        command_list[cl_count].push(i);
    }

    let mut command_list = command_list.into_iter().peekable();

    while let Some(x) = command_list.next() {
        match &x[0][..] {
            "cd" => {
                let new_dir = if x.len() == 1 {
                    std::env::var("HOME").unwrap_or(String::from("/")).into()
                } else {
                    match &x[1][..] {
                        "-" => env.previous_dir.clone(),
                        x => x.to_string().into(),
                    }
                };

                env.previous_dir = env::current_dir()
                    .map_err(|err| {
                        eprintln!("Error: couldn't get previous directory: {err}");
                        PathBuf::from("/")
                    })
                    .unwrap();

                let root = Path::new(&new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                    env.previous_code = 1;
                } else {
                    env.previous_code = 0;
                }
            }
            "history" => {
                let mut history_copy = env.history.clone();
                history_copy.make_contiguous();
                for command in history_copy {
                    println!("{}", command);
                }
            }
            "previous" => println!("{}", env.previous_code),
            "exit" => return Some(1),
            command => {
                if command == "".to_string() {
                    return None;
                }

                // map stdin
                let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                    Stdio::from(output.stdout.unwrap())
                });
                // set stdout
                let stdout = if command_list.peek().is_some() {
                    Stdio::piped()
                } else {
                    Stdio::inherit()
                };

                let child = Command::new(command)
                    .args(&x[1..])
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                match child {
                    Ok(output) => previous_command = Some(output),
                    Err(e) => {
                        previous_command = None;
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    }

    if let Some(mut final_command) = previous_command {
        match final_command.wait().expect("something").code() {
            Some(code) => env.previous_code = code,
            None => {
                env.previous_code = 999;
            }
        }
    }
    return None;
}
