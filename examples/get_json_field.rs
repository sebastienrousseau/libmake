// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! # Test: Retrieving a Field from a CSV File
//!
//! This is a test that demonstrates how to retrieve a specific field from a CSV file
//! using the `get_csv_field` function from the `libmake` crate.
//!
//! ## Purpose
//!
//! The purpose of this test is to show how to extract a CSV field (`field_title`)
//! from a CSV file located at the specified path (`file_path`).
//!
//! ## Usage
//!
//! To run this test, ensure that you have a valid CSV file at the specified path.
//! The test checks if the file exists and then uses the `get_csv_field` function
//! to retrieve the specified CSV field. If the file exists and the field is found,
//! it prints the field's value; otherwise, it prints an error message or an empty string.
//!
//! ```rust
//! // Import the necessary function for retrieving a field from a CSV file.
//! use libmake::utils::get_csv_field;
//! use std::path::Path;
//!
//! // Specify the path to the CSV file.
//! let file_path = "../tests/data/mylibrary.csv";
//!
//! // Define the CSV field to retrieve.
//! let field_title = "title";
//!
//! // Check if the CSV file exists before retrieving the field.
//! let value = if Path::new(file_path).exists() {
//!     // If the file exists, use the `get_csv_field` function to retrieve the field.
//!     match get_csv_field(Some(file_path), 0) {
//!         Some(values) => values.join(", "),
//!         None => {
//!             eprintln!("Error retrieving field: {}", field_title);
//!             String::new()
//!         }
//!     }
//! } else {
//!     // If the file doesn't exist, set the value to an empty string.
//!     String::new()
//! };
//!
//! // Print the result.
//! println!("🦀 get_csv_field, ✅ {}: {}", field_title, value);
//! ```

// Title: Test: Retrieving a field from a CSV file
use libmake::utils::get_csv_field;
use std::path::Path;

/// Retrieve CSV field
///
/// # Arguments
///
/// * `file_path` - Path to the CSV file
/// * `field_title` - Name of the CSV field to retrieve
///
/// # Returns
///
/// The value of the CSV field, or an empty string if the file does not exist or the field cannot be found
///
pub(crate) fn main() {
    // Retrieve CSV field
    let file_path = "../tests/data/mylibrary.csv";
    let field_title = "title";

    let value = if Path::new(file_path).exists() {
        match get_csv_field(Some(file_path), 0) {
            Some(values) => values.join(", "),
            None => {
                eprintln!("Error retrieving field: {field_title}");
                String::new()
            }
        }
    } else {
        String::new()
    };

    println!("🦀 get_csv_field, ✅ {field_title}: {value}");
}
