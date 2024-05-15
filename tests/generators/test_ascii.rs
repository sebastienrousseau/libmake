#[cfg(test)]
mod tests {
    use libmake::{
        generators::ascii::{generate_ascii_art, load_standard_font},
        macro_ascii,
        models::error_ascii_art::AsciiArtError,
    };

    #[test]
    fn test_generate_ascii_art_success() {
        let text = "Hello, world!";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }

    #[test]
    fn test_generate_ascii_art_empty_text() {
        let text = "";
        let result = generate_ascii_art(text);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AsciiArtError::ConversionError
        ));
    }

    #[test]
    fn test_load_standard_font_success() {
        let result = load_standard_font();
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_ascii_art_conversion_error() {
        let text = "\u{1F600}"; // Emoji character
        let result = generate_ascii_art(text);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AsciiArtError::ConversionError
        ));
    }

    #[test]
    fn test_generate_ascii_art_multiple_lines() {
        let text = "Hello,\nworld!";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(ascii_art.contains('\n'));
    }

    #[test]
    fn test_generate_ascii_art_special_characters() {
        let text = "!@#$%^&*()_+";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }

    #[test]
    fn test_macro_ascii_success() {
        let ascii_art = macro_ascii!("Hello, world!");
        assert!(!ascii_art.is_empty());
    }

    #[test]
    #[should_panic(
        expected = "Failed to generate ASCII art: Failed to convert text to ASCII art"
    )]
    fn test_macro_ascii_empty_text() {
        let _ = macro_ascii!("");
    }

    #[test]
    #[should_panic(
        expected = "Failed to generate ASCII art: Failed to convert text to ASCII art"
    )]
    fn test_macro_ascii_conversion_error() {
        let _ = macro_ascii!("\u{1F600}"); // Emoji character
    }

    #[test]
    fn test_macro_ascii_multiple_lines() {
        let ascii_art = macro_ascii!("Hello,\nworld!");
        assert!(ascii_art.contains('\n'));
    }

    #[test]
    fn test_macro_ascii_special_characters() {
        let ascii_art = macro_ascii!("!@#$%^&*()_+");
        assert!(!ascii_art.is_empty());
    }
}
