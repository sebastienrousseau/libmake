//! # Test: Retrieving a CSV Field
//!
//! This is a test that demonstrates how to retrieve a specific field from a CSV file
//! using the `get_csv_field` function from the `libmake` crate.
//!
//! ## Purpose
//!
//! The purpose of this test is to show how to extract a CSV field at a specified index
//! from a CSV file located at the specified path (`file_path`).
//!
//! ## Usage
//!
//! To run this test, ensure that you have a valid CSV file at the specified path.
//! The test uses the `get_csv_field` function to retrieve the field at the specified index (0).
//! It then prints the result.
//!
//! ```rust
//! // Import the necessary function for retrieving a field from a CSV file.
//! use libmake::utils::get_csv_field;
//!
//! // Specify the path to the CSV file.
//! let file_path = "../tests/data/mylibrary.csv";
//!
//! // Retrieve the CSV field at index 0 and print the result.
//! println!(
//!     "🦀 get_csv_field, ✅ {:?}",
//!     get_csv_field(Some(file_path), 0)
//! );
//! ```

// Title: Test: Retrieving a CSV Field
use libmake::utils::get_csv_field;

pub fn main() {
    // Retrieve CSV field
    let file_path = "../tests/data/mylibrary.csv";
    println!(
        "🦀 get_csv_field, ✅ {:?}",
        get_csv_field(Some(file_path), 0)
    );
}
