// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

//! # Libmake Application
//!
//! This is the main entry point for the cmn application.
//! Call the `run()` function from the `LibMake (LM)` module.
//!
//! # Purpose
//! The purpose of this function is to serve as the entry point for the `cmn` application.
//! It calls the `run` function from the `LibMake (LM)` crate to execute the desired tasks.
//!
//! # Usage
//! To use the `cmn` application, you can include it as part of your Rust project.
//! The `main` function of the application calls the `run` function from the `LibMake (LM)` module.
//! If an error occurs during execution, it prints an error message and exits with a non-zero status code.
//!
//! ```rust
//! use libmake::run;
//!
//!/ This is the main entry point for the cmn application.
//! Call the `run()` function from the `LibMake (LM)` module.
//! if let Err(ref e) = run() {
//!     eprintln!("Error running cmn: {}", e);
//!     std::process::exit(1);
//! }
//! ```
//!
//! This application allows you to interact with and use the functionality provided by the `LibMake (LM)` crate.

use libmake::run;

/// This is the main entry point for the cmn application.
/// Call the `run()` function from the `LibMake (LM)` module.
///
/// # Parameters
/// This function does not take any parameters.
///
/// # Returns
/// This function returns a `Result` type. If the `run` function from the `LibMake (LM)` module succeeds, it returns `Ok(())`. If an error occurs, it returns `Err(err)`, where `err` is an instance of the `Error` type.
///
/// # Panics
/// This function may panic if the `run` function from the `LibMake (LM)` module returns an error and the error is not handled properly.
///
/// # Examples
/// ```rust
/// use libmake::run;
///
/// This is the main entry point for the cmn application.
/// Call the `run()` function from the `LibMake (LM)` module.
/// if let Err(err) = run() {
///     eprintln!("Error running cmn: {}", err);
///     std::process::exit(1);
/// }
/// ```
///
/// # See Also
/// - [`run` function from the `LibMake (LM)` module](crate::libmake::run)
///
/// # Author
/// Copyright © 2023-2024 LibMake. All rights reserved.
///
/// # License
/// This code is dual-licensed under the Apache License 2.0 and the MIT License.

fn main() {
    // Call the `run()` function from the `LibMake (LM)` module.
    if let Err(err) = run() {
        eprintln!("Error running cmn: {}", err);
        std::process::exit(1);
    }
}
