// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

/// A macro for generating ASCII art from text.
///
/// This macro takes a string literal as input and generates ASCII art using the `generate_ascii_art` function.
/// If the conversion is successful, the macro returns the ASCII art as a string.
/// If an error occurs during the conversion, the macro panics with an error message.
///
/// # Examples
///
/// ```
/// use libmake::macro_ascii;
///
/// let art = macro_ascii!("Hello, world!");
/// println!("{}", art);
/// ```
#[macro_export]
macro_rules! macro_ascii {
    ($text:expr) => {
        match $crate::generators::ascii::generate_ascii_art($text) {
            Ok(art) => art,
            Err(err) => panic!("Failed to generate ASCII art: {}", err),
        }
    };
}
