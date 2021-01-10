fn main() {
    let config = rsblocks::load_config().unwrap();
    let blocks = rsblocks::Blocks::new();
    rsblocks::run(config, blocks);
}
