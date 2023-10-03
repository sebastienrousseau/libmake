//! # Example: Generating Templates from a Configuration File
//!
//! This is an example that demonstrates how to generate template files
//! based on a configuration file using the `generate_from_config` function.
//!
//! ## Usage
//!
//! To run this example, make sure you have a valid configuration file at the specified path.
//! The example allows you to define the file type (e.g., "yaml") and the file path.
//! It then calls the `generate_from_config` function with the file path and file type as parameters.
//!
//! If generation is successful, it does nothing (the template files are created).
//! If there is an error during generation, it prints an error message.
//!
//! ```rust
//! use libmake::generator::generate_from_config;
//!
//!   // Define the file path for the configuration file.
//!   let file_path = "./tests/data/mylibrary.yaml";
//!
//!   // Define the file type, which is "yaml" in this case.
//!   let file_type = "yaml";
//!
//!   // Call the generate_from_config function with the file_path and file_type.
//!   // This function generates template files based on the configuration.
//!   match generate_from_config(file_path, file_type) {
//!       // If generation is successful, do nothing (the template files are created).
//!       Ok(_) => (),
//!       // If there is an error during generation, print an error message.
//!       Err(err) => eprintln!("Error: {}", err),
//!   }
//! ```

use libmake::generator::generate_from_config;

fn main() {
    // Define the file path for the configuration file.
    let file_path = "./tests/data/mylibrary.yaml";

    // Define the file type, which is "yaml" in this case.
    let file_type = "yaml";

    // Call the generate_from_config function with the file_path and file_type.
    // This function generates template files based on the configuration.
    match generate_from_config(file_path, file_type) {
        // If generation is successful, do nothing (the template files are created).
        Ok(_) => (),
        // If there is an error during generation, print an error message.
        Err(err) => eprintln!("Error: {}", err),
    }
}
