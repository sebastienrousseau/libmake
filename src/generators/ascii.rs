// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! This module provides functionality for generating ASCII art from text using the FIGlet library.

use figlet_rs::FIGfont;
use crate::models::error_ascii_art::AsciiArtError;

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
/// use libmake::generators::ascii::generate_ascii_art;
///
/// let text = "Hello, world!";
/// let result = generate_ascii_art(text);
/// assert!(result.is_ok());
/// ```
pub fn generate_ascii_art(text: &str) -> Result<String, AsciiArtError> {
    if text.is_empty() {
        return Err(AsciiArtError::ConversionError);
    }

    let standard_font = load_standard_font()?;
    let figure = standard_font
        .convert(text)
        .ok_or(AsciiArtError::ConversionError)?;

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
/// use libmake::generators::ascii::load_standard_font;
///
/// let result = load_standard_font();
/// assert!(result.is_ok());
/// ```
pub fn load_standard_font() -> Result<FIGfont, AsciiArtError> {
    FIGfont::standard().map_err(|_| AsciiArtError::FontLoadError)
}
