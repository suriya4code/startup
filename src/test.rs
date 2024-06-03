#[cfg(test)]
mod tests {
    fn format_message(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    #[test]
    fn test_format_message() {
        assert_eq!(format_message("Suriya"), "Hello, Suriya!");
        assert_eq!(format_message("GitHub Copilot"), "Hello, GitHub Copilot!");
    }
}