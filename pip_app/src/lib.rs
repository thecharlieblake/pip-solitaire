//!
//! # Crate test
//!
//! Testing
//!
//! ## Well...
//!
//! This is some more doc text
//!

///
/// # An example of a library module
///
/// This is just some test doc
///
/// ```
/// use pip_app::lib_mod::hello_world;
///
/// assert_eq!(hello_world(), "Hello World!");
/// ```
///
pub mod lib_mod {
    ///
    /// fn comment for *hello world* fn
    ///
    pub fn hello_world() -> String {
        String::from("Hello World!")
    }
}

#[cfg(test)]
mod tests {
    use super::lib_mod::*;

    #[test]
    fn unit_test() {
        assert_eq!(hello_world(), "Hello World!");
    }
}
