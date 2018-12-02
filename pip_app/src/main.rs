#[macro_use]
extern crate clap;
extern crate pip_app;
extern crate pip_core;

use clap::App;
use pip_app::{run, pip_core::{Options, game::pack::Rank}};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let seed = value_t_or_exit!(matches, "deal", u64);
    let max_rank = value_t!(matches, "max-rank", Rank).unwrap_or_else(|e| e.exit());
    
    run(Options {seed, max_rank} )
}
