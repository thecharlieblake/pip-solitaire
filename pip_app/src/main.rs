extern crate pip_app;

use pip_app::lib_mod::hello_world;

fn main() {
    let s = hello_world();
    println!("{}", s);
}
