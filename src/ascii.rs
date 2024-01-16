// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use figlet_rs::FIGfont;
use std::error::Error;
use std::fmt;

/// Error type for ASCII art generation failures.
#[derive(Debug)]
pub enum ArtError {
    /// Represents a failure to load the FIGfont.
    FontLoadError,
    /// Represents a failure to convert text to ASCII art.
    ConversionError,
}

impl fmt::Display for ArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FontLoadError => {
                write!(f, "Failed to load FIGfont")
            }
            Self::ConversionError => {
                write!(f, "Failed to convert text to ASCII art")
            }
        }
    }
}

impl Error for ArtError {}

/// Generates ASCII art from the given text using the standard `FIGfont`.
///
/// # Arguments
///
/// * `text` - The text to convert to ASCII art.
///
/// # Errors
///
/// This function returns an `Err` in the following situations:
///
/// - If the standard `FIGfont` fails to load (`FontLoadError`).
/// - If the text cannot be converted to ASCII art (`ConversionError`).
///
pub fn generate_ascii_art(text: &str) -> Result<String, ArtError> {
    let standard_font =
        FIGfont::standard().map_err(|_| ArtError::FontLoadError)?;
    let figure = standard_font
        .convert(text)
        .ok_or(ArtError::ConversionError)?;
    Ok(figure.to_string())
}
