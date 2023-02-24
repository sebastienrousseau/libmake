extern crate figlet_rs;
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
/// # Examples
///
/// ```
/// use libmake::ascii;
/// ascii::generate_ascii_art("LibMake", "./resources/standard.flf");
/// ```
///
pub fn generate_ascii_art(text: &str, font_file: &str) {
    let font_file = font_file;
    let small_font = FIGfont::from_file(font_file).unwrap();
    let figure = small_font.convert(text);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}
