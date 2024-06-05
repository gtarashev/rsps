/***        modules             ***/
mod commands;
mod environment;
mod input;
mod config;

/***        imports             ***/
use commands::process_command;
use input::std_read_into_buffer;
use config::{initialise, reset_term};
use std::io::{stdout, Write};

/***        methods             ***/
fn rsps_loop(env: &mut environment::Environment) {
    let mut input = String::new();
    let mut complete = true;

    print!("{}", env.ps1);
    if let Err(e) = stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
    }

    let mut popped = 0; /* used if a character has been deleted */
    let mut buffer;
    let mut buffer_size;
    let mut character;
    loop {
        (buffer_size, buffer) = std_read_into_buffer();
        if buffer_size > 1 {
            continue;
        }
        character = buffer[0] as char;

        match character {
            '\n' => {
                complete = true;
                input = format!("{}{}", input, '\n');
            },
            '\u{3}' => { /* CTRL + C */
                env.previous_code = 1;
                println!();
                input.clear();
                complete = false;
            },
            '\u{7f}' => { /* backspace */
                input.pop();
                popped = 1;    
            }
            '\0' => (), /* read timed out */
            _ => {
                input = format!("{}{}", input, character);
            },
        }
        
        // put cursor at the beginning of the line. this means that the next print will overwrite
        // the previous one and appear as if we aren't re-printing every time read_character returns,
        // finally, everything right of the cursor is cleared, this is needed so backspace works
        // correctly
        print!("\x1b[{}D\x1b[K", env.ps1.len() + input.len() + popped);
        popped = 0;
        print!("{}{}", env.ps1, input);
        if let Err(e) = stdout().flush() {
            eprintln!("Error flushing stdout: {}", e);
        }

        if !complete {
            continue;
        }

        match process_command(env, &input) {
            Some(1) => break,
            _ => (),
        }
        // takes a little while for the next line to appear if we wait for input timeout, so we
        // print again and flush after commands to ensure that doesnt happen.
        print!("\x1b[{}D\x1b[K", env.ps1.len() + input.len() + popped);
        print!("{}", env.ps1);
        if let Err(e) = stdout().flush() {
            eprintln!("Error flushing stdout: {}", e);
        }

        input.clear();
        complete = false;
    }
}

/***        main                ***/
fn main() {
    let mut env = initialise();
    rsps_loop(&mut env);
    reset_term(&env);
}
