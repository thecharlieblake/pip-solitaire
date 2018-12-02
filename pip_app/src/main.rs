#[macro_use]
extern crate clap;
extern crate pip_app;

use clap::App;
use pip_app::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let seed = value_t_or_exit!(matches, "deal", u64);
    
    run(seed)
}
