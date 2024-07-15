/***        modules             ***/
mod commands;
mod environment;
mod input;
mod config;
mod rsps_loop;
mod output;
mod keymaps;

/***        imports             ***/
use config::initialise;

/***        main                ***/
fn main() {
    initialise();
}
