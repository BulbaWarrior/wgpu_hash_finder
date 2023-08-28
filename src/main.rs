fn main() {
    pretty_env_logger::init();
    pollster::block_on(wgpu_miner::run()).unwrap();
}
