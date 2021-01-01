use rsblocks;

fn main() {
    let config = rsblocks::load_config().unwrap();
    rsblocks::run(config);
}
