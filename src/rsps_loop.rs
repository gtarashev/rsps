use crate::commands::process_command;
use crate::input::std_read_into_buffer;
use crate::environment;
use crate::output::{print_line, clear_line};
use crate::keymaps::*;

/***        methods             ***/
pub fn shell_loop(env: &mut environment::Environment) {
    let mut input = String::new();
    let mut complete = false;

    print_line(&mut env.stdout_handle, &env.ps1, &String::new());

    let mut popped = 0; /* used if a character has been deleted */
    let mut buffer = [0; 3];
    loop {
        _ = std_read_into_buffer(&mut env.stdin_handle, &mut buffer); /* we dont use the buffer_size */
        match buffer {
            NEWLINE => {
                complete = true;
                input = format!("{}{}", input, '\n');
            },
            CTRL_C => {
                env.previous_code = 1;
                input.push('\n');
                print_line(&mut env.stdout_handle, &env.ps1, &input);
                input.clear();
                complete = false;
            },
            BACKSPACE => {
                input.pop();
                popped = 1;    
            },
            READ_TIMEOUT => (),
            [x, 0, 0] => {
                input = format!("{}{}", input, x as char);
            },
            _ => (), /* escape sequence not implemented */
        }
        // empty the buffer
        buffer = [0; 3];
        
        clear_line(&mut env.stdout_handle, env.ps1.len() + input.len() + popped);
        popped = 0;
        print_line(&mut env.stdout_handle, &env.ps1, &input);

        if !complete {
            continue;
        }
        complete = false;

        // exit command used
        if Some(1) ==  process_command(env, &input) {
            break;
        }

        // if we wait for the next loop iteration to print it would have to
        // wait for the read to return, so there is some delay, therefore,
        // reprint the ps1 to combat that
        input.clear();
        print_line(&mut env.stdout_handle, &env.ps1, &input);
    }
}

