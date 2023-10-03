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

/// This is a simple test for generating files from a CSV file using the `generate_from_csv` function.
/// It attempts to generate template files from a CSV file and expects the operation to be successful.
fn main() {
    // Define the path to the CSV file to be used for testing.
    let csv_file_path = "./tests/data/mylibrary.csv";

    // Attempt to generate template files from the specified CSV file.
    // If successful, it indicates that the generation process worked as expected.
    generate_from_csv(csv_file_path)
        .expect("Failed to generate the template files");
}
