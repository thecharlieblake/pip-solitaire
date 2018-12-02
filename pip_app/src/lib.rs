pub extern crate pip_core;

use pip_core::{utils, gen_game, Options};

pub fn run(ops: Options) {
    println!("{}", utils::yaml::to_pretty_string(&gen_game(ops)))
}