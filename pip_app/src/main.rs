#[macro_use]
extern crate clap;
use clap::App;

extern crate pip_app;
use pip_app::app_lib::*;

fn main() -> Result<(), ()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    run(matches)
}
