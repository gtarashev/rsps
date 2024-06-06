use std::io::{stdout, Write};
use crate::commands::process_command;
use crate::input::std_read_into_buffer;
use crate::environment;
use crate::output::display_ps1;

/***        methods             ***/
pub fn shell_loop(env: &mut environment::Environment) {
    let mut input = String::new();
    let mut complete = true;

    display_ps1(&env.ps1);

    let mut popped = 0; /* used if a character has been deleted */
    let mut buffer = [0; 3];
    loop {
        _ = std_read_into_buffer(&mut buffer); /* we dont use the buffer_size */
        match buffer {
            [10, 0, 0] => { /* newline */
                complete = true;
                input = format!("{}{}", input, '\n');
            },
            [3, 0, 0] => { /* CTRL + C */
                env.previous_code = 1;
                println!();
                input.clear();
                complete = false;
            },
            [127, 0, 0] => { /* backspace */
                input.pop();
                popped = 1;    
            }
            [0, 0, 0] => (), /* read timed out */
            [x, 0, 0] => {
                input = format!("{}{}", input, x as char);
            },
            _ => (), /* escape sequence not implemented */
        }
        // empty the buffer
        buffer = [0; 3];
        
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
        display_ps1(&env.ps1);

        input.clear();
        complete = false;
    }
}

