///
/// # Library module for pip_app
///

extern crate clap;

pub mod app_lib {

    use clap::ArgMatches;

    use std::result::Result::Ok;

    ///
    /// Runs the main application
    ///
    pub fn run(_matches: ArgMatches) -> Result<(), ()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::app_lib::*;

    use clap::ArgMatches;

    #[test]
    fn run_returns_ok() {
        assert_eq!(Ok(()), run(ArgMatches::default()));
    }
}
