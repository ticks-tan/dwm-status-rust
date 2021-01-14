mod config;
mod run;
mod types;
mod utils;

fn main() {
    let config = config::load_config().unwrap();
    let blocks = types::Blocks::new();
    run::run(config, blocks);
}
