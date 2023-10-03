// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use figlet_rs::FIGfont;

/// Generates ASCII art from the given text using the specified FIGfont
/// file.
///
/// # Arguments
///
/// - `text` - The text to convert to ASCII art in the form of a string.
/// - `font_file` - The path to the FIGfont file to use for the
/// conversion in the form of a string.
///
/// # Panics
///
/// This function panics if the FIGfont file cannot be loaded or if the
/// conversion from text to ASCII art fails.
///
pub fn generate_ascii_art(text: &str) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(text);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}
