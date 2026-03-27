pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn greet_preserves_input_case() {
        assert_eq!(greet("world"), "Hello, world!");
    }
}
