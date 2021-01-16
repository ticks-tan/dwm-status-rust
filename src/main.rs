mod config;
mod run;
mod types;
mod utils;
use std::env;
use std::process;

fn main() {
    // if X display is not found then exit the program
    if env::var("DISPLAY").is_err() {
        eprintln!("Error: No Display Running!");
        process::exit(1);
    };
    let config = config::load_config().unwrap();
    let blocks = types::Blocks::new();
    run::run(config, blocks);
}
