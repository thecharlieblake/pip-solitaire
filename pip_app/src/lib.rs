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
/// assert_eq!(hello_world(), "Xello World!");
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
    use super::*;

    #[test]
    fn unit_test() {
        assert_eq!(hello_world(), "Hello World!");
    }
}
