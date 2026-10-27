// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2026 LibMake. All rights reserved.

//! Golden tests for `generate_ascii_art`. Each file under `tests/golden/ascii`
//! is the output of figlet-rs 1.0.0's standard font for the same text,
//! captured before the in-crate renderer replaced that dependency.

use libmake::generators::ascii::{
    generate_ascii_art, load_standard_font,
};
use libmake::models::error_ascii_art::AsciiArtError;

#[test]
fn renders_like_figlet_rs() {
    let cases = [
        ("LibMake", include_str!("golden/ascii/libmake.txt")),
        (
            "Hello, world!",
            include_str!("golden/ascii/hello_world.txt"),
        ),
        (
            "Hello,\nworld!",
            include_str!("golden/ascii/hello_world_two_lines.txt"),
        ),
        ("!@#$%^&*()_+", include_str!("golden/ascii/punctuation.txt")),
    ];
    for (text, expected) in cases {
        assert_eq!(
            generate_ascii_art(text).unwrap(),
            expected,
            "{text:?}"
        );
    }
}

#[test]
fn rejects_text_the_font_cannot_render() {
    assert!(matches!(
        generate_ascii_art(""),
        Err(AsciiArtError::ConversionError)
    ));
    assert!(matches!(
        generate_ascii_art("\u{1F600}"),
        Err(AsciiArtError::ConversionError)
    ));
}

#[test]
fn loads_the_bundled_font() {
    let font = load_standard_font().unwrap();
    assert_eq!(
        font.convert("LibMake").unwrap().to_string(),
        include_str!("golden/ascii/libmake.txt")
    );
}
