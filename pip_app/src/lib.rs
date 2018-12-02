extern crate pip_core;

use pip_core::{utils, gen_game};

pub fn run(seed: u64) {
    println!("{}", utils::yaml::to_pretty_string(&gen_game(seed)).unwrap())
}