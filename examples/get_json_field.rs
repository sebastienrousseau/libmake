// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! # Test: Retrieving a Field from a JSON File
//!
//! This is a test that demonstrates how to retrieve a specific field from a JSON file
//! using the `get_json_field` function from the `libmake` crate.
//!
//! ## Purpose
//!
//! The purpose of this test is to show how to extract a JSON field (`field_author`)
//! from a JSON file located at the specified path (`file_path`).
//!
//! ## Usage
//!
//! To run this test, ensure that you have a valid JSON file at the specified path.
//! The test checks if the file exists and then uses the `get_json_field` function
//! to retrieve the specified JSON field. If the file exists and the field is found,
//! it prints the field's value; otherwise, it prints an empty string.
//!
//! ```rust
//! // Import the necessary function for retrieving a field from a JSON file.
//! use libmake::utils::get_json_field;
//! use std::path::Path;
//!
//!  // Specify the path to the JSON file.
//!  let file_path = "../tests/data/mylibrary.json";
//!
//!  // Define the JSON field to retrieve.
//!  let field_author = "author";
//!
//!  // Check if the JSON file exists before retrieving the field.
//!  let value = if Path::new(file_path).exists() {
//!      // If the file exists, use the `get_json_field` function to retrieve the field.
//!      get_json_field(Some(file_path), field_author)
//!  } else {
//!      // If the file doesn't exist, set the value to an empty string.
//!      String::new()
//!  };
//
//!  // Print the result.
//!  println!("🦀 get_json_field, ✅ {}: {}", field_author, value);
//! ```

// Title: Test: Retrieving a field from a JSON file
use libmake::utils::get_json_field;
use std::path::Path;

/// Retrieve JSON field
///
/// # Arguments
///
/// * `file_path` - Path to the JSON file
/// * `field_author` - Name of the JSON field to retrieve
///
/// # Returns
///
/// The value of the JSON field, or an empty string if the file does not exist or the field cannot be found
///
pub(crate) fn main() {
    // Retrieve JSON field
    let file_path = "../tests/data/mylibrary.json";
    let field_author = "author";
    let value = if Path::new(file_path).exists() {
        get_json_field(Some(file_path), field_author)
    } else {
        String::new()
    };
    println!("🦀 get_json_field, ✅ {field_author}: {value}");
}
