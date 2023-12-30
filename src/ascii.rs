// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use figlet_rs::FIGfont;
use std::error::Error;
use std::fmt;

/// Error type for ASCII art generation failures.
#[derive(Debug)]
pub enum AsciiArtError {
    /// Represents a failure to load the FIGfont.
    FontLoadError,
    /// Represents a failure to convert text to ASCII art.
    ConversionError,
}

impl fmt::Display for AsciiArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AsciiArtError::FontLoadError => {
                write!(f, "Failed to load FIGfont")
            }
            AsciiArtError::ConversionError => {
                write!(f, "Failed to convert text to ASCII art")
            }
        }
    }
}

impl Error for AsciiArtError {}

/// Generates ASCII art from the given text using the standard FIGfont.
///
/// # Arguments
///
/// * `text` - The text to convert to ASCII art.
///
/// # Returns
///
/// This function returns a `Result` with the ASCII art as `String` or
/// an `AsciiArtError` if the operation fails.
///
pub fn generate_ascii_art(text: &str) -> Result<String, AsciiArtError> {
    let standard_font = FIGfont::standard()
        .map_err(|_| AsciiArtError::FontLoadError)?;
    let figure = standard_font
        .convert(text)
        .ok_or(AsciiArtError::ConversionError)?;
    Ok(figure.to_string())
}
