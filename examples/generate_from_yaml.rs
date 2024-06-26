// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

//! # Example: Generating Templates from a YAML File
//!
//! This is a simple example that demonstrates how to use the `generate_from_yaml` function
//! from the `libmake` crate to generate template files from a YAML file.
//!
//! ## Usage
//!
//! To run this example, make sure you have a valid YAML file at the specified path.
//! The example specifies the path to the YAML file (`yaml_file_path`) and then calls
//! the `generate_from_yaml` function with this path as a parameter.
//!
//! If the generation process is successful, it does nothing (the template files are created).
//! If there is an error during generation, it prints an error message.
//!
//! ```rust
//! // Import the necessary function for generating templates from a YAML file.
//! use libmake::generator::generate_from_yaml;
//!
//! // Specify the path to the YAML file to be used for generating templates.
//! let yaml_file_path = "./tests/data/mylibrary.yaml";
//!
//! // Generate template files from the specified YAML file.
//! generate_from_yaml(yaml_file_path)
//!     .expect("Failed to generate the template files");
//! ```

// Import the necessary function for generating templates from a YAML file.
use libmake::generators::yaml::generate_from_yaml;

/// Generate template files from the specified YAML file.
///
/// # Arguments
///
/// * `yaml_file_path` - Path to the YAML file to be used for generating templates.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if the template files are generated successfully, or returns an error message if there is an error during generation.
///
pub(crate) fn main() {
    // Specify the path to the YAML file to be used for generating templates.
    let yaml_file_path = "./tests/data/mylibrary.yaml";

    // Generate template files from the specified YAML file.
    generate_from_yaml(yaml_file_path)
        .expect("Failed to generate the template files");
}
