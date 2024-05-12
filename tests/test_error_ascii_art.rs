#[cfg(test)]
mod tests {
    use std::error::Error;
    use libmake::models::error_ascii_art::AsciiArtError;

    #[test]
    fn test_ascii_art_error_font_load_error() {
        let error = AsciiArtError::FontLoadError;
        assert_eq!(error.to_string(), "Failed to load FIGfont");
    }

    #[test]
    fn test_ascii_art_error_conversion_error() {
        let error = AsciiArtError::ConversionError;
        assert_eq!(error.to_string(), "Failed to convert text to ASCII art");
    }

    #[test]
    fn test_ascii_art_error_partial_eq() {
        let error1 = AsciiArtError::FontLoadError;
        let error2 = AsciiArtError::FontLoadError;
        let error3 = AsciiArtError::ConversionError;
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_ascii_art_error_source() {
        let error = AsciiArtError::FontLoadError;
        assert_eq!(error.source().is_none(), true);
    }

    #[test]
    fn test_ascii_art_error_debug() {
        let error = AsciiArtError::FontLoadError;
        let debug_output = format!("{:?}", error);
        assert_eq!(debug_output, "FontLoadError");
    }

    #[test]
    fn test_ascii_art_error_clone() {
        let error = AsciiArtError::FontLoadError;
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);
    }
}
