mod test;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = format_message("Suriya");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

}

fn format_message(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[test]
fn test_format_message() {
    assert_eq!(format_message("Suriya"), "Hello, Suriya!");
    assert_eq!(format_message("Rustacean"), "Hello, Rustacean!");
}