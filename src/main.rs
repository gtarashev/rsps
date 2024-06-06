/***        modules             ***/
mod commands;
mod environment;
mod input;
mod config;
mod rsps_loop;
mod output;
mod keymaps;

/***        imports             ***/
use config::{initialise, reset_term};
use rsps_loop::shell_loop;


/***        main                ***/
fn main() {
    let mut env = initialise();
    shell_loop(&mut env);
    reset_term(&env);
}
