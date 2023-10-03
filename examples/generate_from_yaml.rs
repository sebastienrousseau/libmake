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
use libmake::generator::generate_from_yaml;

fn main() {
    // Specify the path to the YAML file to be used for generating templates.
    let yaml_file_path = "./tests/data/mylibrary.yaml";

    // Generate template files from the specified YAML file.
    generate_from_yaml(yaml_file_path)
        .expect("Failed to generate the template files");
}