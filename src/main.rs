mod game;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    game::connect_4();
}