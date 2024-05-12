// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

use std::error::Error;
use std::fmt;

/// Error type for ASCII art generation failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsciiArtError {
    /// Represents a failure to load the FIGfont.
    FontLoadError,
    /// Represents a failure to convert text to ASCII art.
    ConversionError,
}

impl fmt::Display for AsciiArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FontLoadError => write!(f, "Failed to load FIGfont"),
            Self::ConversionError => {
                write!(f, "Failed to convert text to ASCII art")
            }
        }
    }
}

impl Error for AsciiArtError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
