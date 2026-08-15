#[cfg(test)]
mod tests {
    use libmake::macro_ascii;

    #[test]
    fn test_macro_ascii_success() {
        let art = macro_ascii!("Hi");
        // figlet-rs 1.0.0 kerns more tightly than 0.1.x: the leading
        // column of padding each glyph used to carry is gone.
        assert_eq!(art, " _   _ _ \n| | | (_)\n| |_| | |\n|  _  | |\n|_| |_|_|\n         \n");
    }

    #[test]
    #[should_panic(
        expected = "Failed to generate ASCII art: Failed to convert text to ASCII art"
    )]
    fn test_macro_ascii_empty_input() {
        let _art = macro_ascii!("");
    }

    #[test]
    #[should_panic(
        expected = "Failed to generate ASCII art: Failed to convert text to ASCII art"
    )]
    fn test_macro_ascii_invalid_input() {
        let _art = macro_ascii!("🦀");
    }
}
