pub fn hello_world() -> String {
    String::from("Hello World!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        assert_eq!(hello_world(), "Hello World!");
    }
}
