// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! This module provides functionality for generating ASCII art from text using the FIGlet library.

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
            Self::FontLoadError => write!(f, "Failed to load FIGfont"),
            Self::ConversionError => write!(f, "Failed to convert text to ASCII art"),
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
/// - If the input `text` is empty (`ConversionError`).
/// - If the standard `FIGfont` fails to load (`FontLoadError`).
/// - If the text cannot be converted to ASCII art (`ConversionError`).
///
/// # Examples
///
/// ```
/// use libmake::ascii::generate_ascii_art;
///
/// let text = "Hello, world!";
/// let result = generate_ascii_art(text);
/// assert!(result.is_ok());
/// ```
pub fn generate_ascii_art(text: &str) -> Result<String, ArtError> {
    if text.is_empty() {
        return Err(ArtError::ConversionError);
    }

    let standard_font = load_standard_font()?;
    let figure = standard_font
        .convert(text)
        .ok_or(ArtError::ConversionError)?;
    Ok(figure.to_string())
}

/// Loads the standard FIGfont.
///
/// # Errors
///
/// This function returns an `Err` if the standard `FIGfont` fails to load (`FontLoadError`).
///
/// # Examples
///
/// ```
/// use libmake::ascii::load_standard_font;
///
/// let result = load_standard_font();
/// assert!(result.is_ok());
/// ```
pub fn load_standard_font() -> Result<FIGfont, ArtError> {
    FIGfont::standard().map_err(|_| ArtError::FontLoadError)
}
