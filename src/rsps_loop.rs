use std::{
    io::{Read, Write},
};

use crate::commands::process_command;
use crate::input::read_into_buffer;
use crate::environment;
use crate::output::{print_line, clear_line};
use crate::keymaps::*;

/***        methods             ***/
pub fn shell_loop<R, W>(env: &mut environment::Environment<R, W>) 
where R: Read,
      W: Write {
    let mut input = String::new();
    let mut complete = false;
    let mut buffer = [0; 3];
    let mut current_command = String::new();
    let mut history_counter = 0;
    let mut history_set = true;

    print_line(&mut env.stdout_handle, &env.ps1, &String::new());

    loop {
        // buffer size is not used
        _ = read_into_buffer(&mut env.stdin_handle, &mut buffer);
        match buffer {
            NEWLINE => {
                complete = true;
                input = format!("{}{}", input, '\n');
            },
            CTRL_C => {
                env.previous_code = 1;
                input.push('\n');
                clear_line(&mut env.stdout_handle);
                print_line(&mut env.stdout_handle, &env.ps1, &input);
                input.clear();
                complete = false;
            },
            BACKSPACE => {
                input.pop();
            },
            ARROW_DOWN => {
                if history_counter != env.history.len() {
                    history_set = false;
                    history_counter += 1;
                }
            },
            ARROW_UP => {
                if history_counter == env.history.len() {
                    current_command = input.clone();
                }

                if history_counter != 0 {
                    history_set = false;
                    history_counter -= 1;
                }
            },
            READ_TIMEOUT => continue,
            [x, 0, 0] => {
                input = format!("{}{}", input, x as char);
            },
            _ => continue, /* escape sequence not implemented */
        }
        // empty the buffer
        buffer = [0; 3];

        if !history_set {
            if history_counter == env.history.len() {
                input = current_command.clone();
            }
            else {
                input = env.history[history_counter].clone();
            }
            history_set = true;
        }
        
        clear_line(&mut env.stdout_handle);
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
        current_command.clear();
        history_counter = env.history.len();
        print_line(&mut env.stdout_handle, &env.ps1, &input);
    }
}

