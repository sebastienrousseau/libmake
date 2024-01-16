//! # Test: Retrieving a Field from a YAML File
//!
//! This is a test that demonstrates how to retrieve a specific field from a YAML file
//! using the `get_yaml_field` function from the `libmake` crate.
//!
//! ## Purpose
//!
//! The purpose of this test is to show how to extract a YAML field (`field_keywords`)
//! from a YAML file located at the specified path (`file_path`). The test specifically
//! handles YAML arrays and converts them into a formatted string.
//!
//! ## Usage
//!
//! To run this test, ensure that you have a valid YAML file at the specified path.
//! The test checks if the file exists and then uses the `get_yaml_field` function
//! to retrieve the specified YAML field (`field_keywords`). If the file exists and
//! the field is found, it processes the field's content (assuming it's an array),
//! formats it into a string, and prints the result. If the file doesn't exist,
//! it prints an empty string.
//!
//! ```rust
//! // Import the necessary function for retrieving a field from a YAML file.
//! use libmake::utils::get_yaml_field;
//! use std::path::Path;
//!
//! // Specify the path to the YAML file.
//! let file_path = "../tests/data/mylibrary.yaml";
//!
//! // Define the YAML field to retrieve.
//! let field_keywords = "keywords";
//!
//! // Check if the YAML file exists before retrieving the field.
//! let value = if Path::new(file_path).exists() {
//!     // If the file exists, use the `get_yaml_field` function to retrieve the field.
//!     let keywords: Vec<String> =
//!         get_yaml_field(Some(file_path), field_keywords)
//!             .split('\n')
//!             .map(|s| s.trim_start_matches("- "))
//!             .filter(|s| !s.is_empty())
//!             .map(|s| format!("\"{}\"", s))
//!             .collect();
//!     format!("[{}]", keywords.join(", "))
//! } else {
//!     // If the file doesn't exist, set the value to an empty string.
//!     String::new()
//! };
//
//! // Print the result.
//! println!("🦀 get_yaml_field, ✅ {}: {}", field_keywords, value);
//! ```

// Title: Test: Retrieving a field from a YAML file
use libmake::utils::get_yaml_field;
use std::path::Path;

pub fn main() {
    let file_path = "../tests/data/mylibrary.yaml";
    let field_keywords = "keywords";

    let value = if Path::new(file_path).exists() {
        let keywords: Vec<String> =
            get_yaml_field(Some(file_path), field_keywords)
                .split('\n')
                .map(|s| s.trim_start_matches("- "))
                .filter(|s| !s.is_empty())
                .map(|s| format!("\"{s}\""))
                .collect();
        format!("[{}]", keywords.join(", "))
    } else {
        String::new()
    };

    println!("🦀 get_yaml_field, ✅ {field_keywords}: {value}");
}
