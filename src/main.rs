/***        modules             ***/
mod commands;
mod config;
mod environment;
mod input;
mod keymaps;
mod lexer;
mod output;
mod rsps_loop;

/***        imports             ***/
use config::initialise;

/***        main                ***/
fn main() {
    initialise();
}
