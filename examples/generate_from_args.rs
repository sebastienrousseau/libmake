// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! A simple test program for the `generate_from_args` function.
//!
//! This program simulates command line arguments and calls the `generate_from_args` function
//! to generate files based on the provided arguments.
//!
//! # Arguments
//!
//! * `--author=<value>` - The author name for the generated files.
//! * `--output=<value>` - The output directory for the generated files.
//!
//! # Example
//!
//! To run this test program, use the following command:
//!
//! ```
//! $ cargo run --release
//! ```
//!
//! Make sure to replace `<value>` with the desired values for `--author` and `--output`.
//!
//! If successful, this program will print "Successfully generated files!".
//!
//! If there is an error, it will print an error message.

// Import the necessary function for generating files from arguments
use libmake::generators::args::generate_from_args;

/// A simple test program for the `generate_from_args` function.
///
/// This program simulates command line arguments and calls the `generate_from_args` function
/// to generate files based on the provided arguments.
///
/// # Arguments
///
/// * `--author=<value>` - The author name for the generated files.
/// * `--output=<value>` - The output directory for the generated files.
///
/// # Example
///
/// To run this test program, use the following command:
///
/// ```
/// $ cargo run --release
/// ```
///
/// Make sure to replace `<value>` with the desired values for `--author` and `--output`.
///
/// If successful, this program will print "Successfully generated files!".
///
/// If there is an error, it will print an error message.
pub(crate) fn main() {
    // Simulate command line arguments
    let args = "--author=Me --output=my_library"
        .split(' ')
        .map(ToString::to_string) // Directly using the method
        .collect::<Vec<String>>();

    // Check if there are at least two arguments (program name and at least one option)
    if args.len() < 2 {
        eprintln!("Usage: {} <args>", args[0]);
        return;
    }

    // Join the arguments (excluding the program name) into a single string
    let args_str = args[1..].join(" ");

    // Call the `generate_from_args` function with the arguments string
    let result = generate_from_args(&args_str);
    println!("{result:?}");

    // Check the result of the function call and print a message accordingly
    match result {
        Ok(()) => println!("Successfully generated files!"),
        Err(err) => eprintln!("Error: {err}"),
    }
}
