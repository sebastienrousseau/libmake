#[cfg(test)]
mod tests {
    use libmake::ascii::{generate_ascii_art, load_standard_font, ArtError};

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
        assert!(matches!(result.unwrap_err(), ArtError::ConversionError));
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
        assert!(matches!(result.unwrap_err(), ArtError::ConversionError));
    }
}
