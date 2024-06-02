/***        modules             ***/
mod commands;
mod environment;
mod input;
mod config;

/***        imports             ***/
use commands::process_command;
use input::get_char;
use config::initialise;
use std::io::{stdout, Write};

/***        methods             ***/
fn rsps_loop(env: &mut environment::Environment) {
    let mut input = String::new();
    let mut complete = true;

    loop {
        if complete {
            print!("{}", &env.ps1);
            if let Err(e) = stdout().flush() {
                eprintln!("Error flushing stdout: {}", e);
            }

            complete = false;
        }

        let char = get_char();

        match char {
            '\n' => {
                complete = true;
                println!();
            },
            _ => {
                input = format!("{}{}", input, char);
                print!("{}", char);
            },
        }
        
        if let Err(e) = stdout().flush() {
            eprintln!("Error flushing stdout: {}", e);
        }

        if complete {
            match process_command(env, &input) {
                Some(1) => break,
                _ => (),
            }
            input.clear();
        }
    }
}

/***        main                ***/
fn main() {
    let mut env = initialise();
    rsps_loop(&mut env);
}
