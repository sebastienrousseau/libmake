// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! # Example: Generating Templates from a CSV File
//!
//! This is a simple test that demonstrates how to generate files from a CSV file
//! using the `generate_from_csv` function. It attempts to generate template files
//! from a CSV file and expects the operation to be successful.
//!
//! ## Usage
//!
//! To run this example, make sure you have a valid CSV file at the specified path.
//! The example will attempt to generate template files based on the CSV data.
//! If generation fails, it will print an error message.
//!
//! ```rust
//! // Import the necessary function for generating templates from a CSV file.
//! use libmake::generator::generate_from_csv;
//!
//! /// This is a simple test for generating files from a CSV file using the `generate_from_csv` function.
//! /// It attempts to generate template files from a CSV file and expects the operation to be successful.
//! // Define the path to the CSV file to be used for testing.
//! let csv_file_path = "./tests/data/mylibrary.csv";
//!
//! // Attempt to generate template files from the specified CSV file.
//! // If successful, it indicates that the generation process worked as expected.
//! generate_from_csv(csv_file_path)
//!     .expect("Failed to generate the template files");
//! ```

// Import the necessary function for generating templates from a CSV file.
use libmake::generator::generate_from_csv;

/// Attempts to generate template files from the specified CSV file.
///
/// # Parameters
///
/// * `csv_file_path` - The path to the CSV file that contains the template generation information.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the template generation process was successful, or returns an error if it failed.
///
/// # Examples
///
/// The following example demonstrates how to use the `generate_from_csv` function:
///
/// ```rust
/// use libmake::generator::generate_from_csv;
///
/// let csv_file_path = "./tests/data/mylibrary.csv";
///
/// // Attempt to generate template files from the specified CSV file.
/// // If successful, it indicates that the generation process worked as expected.
/// generate_from_csv(csv_file_path)
///     .expect("Failed to generate the template files");
/// ```
pub fn main() {
    // Define the path to the CSV file to be used for testing.
    let csv_file_path = "./tests/data/mylibrary.csv";

    // Attempt to generate template files from the specified CSV file.
    // If successful, it indicates that the generation process worked as expected.
    generate_from_csv(csv_file_path)
        .expect("Failed to generate the template files");
}
