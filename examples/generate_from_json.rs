// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! # Example: Generating Templates from a JSON File
//!
//! This example demonstrates how to use the `generate_from_json` function from the `libmake` crate
//! to generate template files from a JSON file containing configuration data.
//!
//! ## Usage
//!
//! To run this example, ensure that you have a valid JSON file at the specified path.
//! The example defines the path to the JSON file (`json_file_path`) and then calls
//! the `generate_from_json` function with this path as a parameter.
//!
//! If the generation process is successful, it does nothing (the template files are created).
//! If there is an error during generation, it prints an error message.
//!
//! ```rust
//! // Import the necessary function for generating templates from a JSON file.
//! use libmake::generator::generate_from_json;
//!
//! /// This test demonstrates how to use the `generate_from_json` function from the `libmake` crate
//! /// to generate template files from a JSON file.
//!
//! // Define the path to the JSON file that contains configuration data.
//! let json_file_path = "./tests/data/mylibrary.json";
//!
//! // Generate template files based on the data in the JSON file.
//! // If the generation process fails, an error message is printed.
//! generate_from_json(json_file_path)
//!     .expect("Failed to generate the template files");
//! ```

// Import the necessary function for generating templates from a JSON file.
use libmake::generator::generate_from_json;

/// Generate template files based on the data in the JSON file.
///
/// # Arguments
///
/// * `json_file_path` - Path to the JSON file that contains the configuration data.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if the template files are generated successfully, or returns an error message if there is an error during generation.
///
pub(crate) fn main() {
    // Define the path to the JSON file that contains configuration data.
    let json_file_path = "./tests/data/mylibrary.json";

    // Generate template files based on the data in the JSON file.
    // If the generation process fails, print an error message.
    if let Err(err) = generate_from_json(json_file_path) {
        eprintln!("Failed to generate the template files: {err}");
    }
}
