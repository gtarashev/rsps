/***        modules             ***/
mod commands;
mod environment;

/***        imports             ***/
use commands::process_command;
use std::io::{stdin, stdout, Write};

/***        methods             ***/
fn rsps_loop(env: &mut environment::Environment) {
    let mut input = String::new();

    loop {
        print!("{}", &env.ps1);
        if let Err(e) = stdout().flush() {
            eprintln!("Error flushing stdout: {}", e);
        }

        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }


        match process_command(env, &input) {
            Some(1) => break,
            _ => (),
        }

        input.clear();
    }
}

/***        main                ***/
fn main() {
    let mut env: environment::Environment = environment::Environment::new(String::from("> "));
    rsps_loop(&mut env);
}
