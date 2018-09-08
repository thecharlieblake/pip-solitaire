extern crate pip_app;

use pip_app::hello_world;

#[test]
fn integration_test() {
    assert_eq!(hello_world(), "Hello World!");
}
