mod config;
mod run;
mod types;
mod utils;
use std::env;
use std::process;

use lazy_static::initialize;

fn main() {
    
    initialize(&config::CONFIG);

    // if X display is not found then exit the program
    if env::var("DISPLAY").is_err() {
        eprintln!("Error: No Display Running!");
        process::exit(1);
    };

    let blocks = types::Blocks::new();
    run::run(blocks);
}
