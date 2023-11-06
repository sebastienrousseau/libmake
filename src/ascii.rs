
// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use figlet_rs::FIGfont;
use std::error::Error;

/// Generates ASCII art from the given text using the standard FIGfont.
///
/// # Arguments
///
/// * `text` - The text to convert to ASCII art.
///
/// # Errors
///
/// This function returns an error if the FIGfont file cannot be loaded or if the
/// conversion from text to ASCII art fails.
///
pub fn generate_ascii_art(text: &str) -> Result<String, Box<dyn Error>> {
    let standard_font = FIGfont::standard()?;
    let figure = standard_font.convert(text).ok_or("Conversion from text to ASCII art failed")?;
    Ok(figure.to_string())
}
