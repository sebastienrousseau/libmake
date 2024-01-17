// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! # Example: Generating Templates from a TOML File
//!
//! This is a simple example that demonstrates how to generate template files
//! based on configuration stored in a TOML file using the `generate_from_toml`
//! function.
//!
//! ## Usage
//!
//! To run this example, make sure you have a valid TOML configuration file
//! at the specified path. The example will attempt to generate template files
//! based on the configuration. If generation fails, it will print an error
//! message.
//!
//! ```rust
//! // Import the necessary function for generating templates from a TOML file.
//! use libmake::generator::generate_from_toml;
//!
//!     // Define the path to the TOML file containing configuration.
//!     let toml_file_path = "./tests/data/mylibrary.toml"
//!     // Generate template files based on the configuration in the TOML file.
//!     // If generation fails, it will print an error message.
//!     generate_from_toml(toml_file_path)
//!         .expect("Failed to generate the template files");
//! ```
//!
// Import the necessary function for generating templates from a TOML file.
use libmake::generator::generate_from_toml;

/// Generate template files based on the configuration in the TOML file.
///
/// # Arguments
///
/// * `toml_file_path` - Path to the TOML file containing the configuration.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if the template files are generated successfully, or returns an error message if generation fails.
///
pub fn main() {
    // Define the path to the TOML file containing configuration.
    let toml_file_path = "./tests/data/mylibrary.toml";

    // Generate template files based on the configuration in the TOML file.
    // If generation fails, it will print an error message.
    generate_from_toml(toml_file_path)
        .expect("Failed to generate the template files");
}
