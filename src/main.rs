// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Libmake Application
//!
//! This is the main entry point for the `libmake` application. It provides a simple command-line
//! interface to execute the functionality provided by the `libmake` crate.
//!
//! ## Purpose
//!
//! The purpose of this application is to serve as the entry point for the `libmake` functionality.
//! It calls the `run` function from the `libmake` crate to execute the desired tasks.
//!
//! ## Usage
//!
//! To use the `libmake` application, you can include it as part of your Rust project.
//! The main function of the application calls the `run` function from the `libmake` module.
//! If an error occurs during execution, it prints an error message and exits with a non-zero status code.
//!
//! ```rust
//! use libmake::run;
//!
//! /// This is the main entry point for the libmake application.
//! // Call the `run()` function from the `libmake` module.
//! if let Err(ref e) = run() {
//!     eprintln!("Error running libmake: {}", e);
//!     std::process::exit(1);
//! }
//! ```
//!
//! This application allows you to interact with and use the functionality provided by the `libmake` crate.

use libmake::run;

/// This is the main entry point for the libmake application.
fn main() {
    // Call the `run()` function from the `libmake` module.
    if let Err(ref e) = run() {
        eprintln!("Error running libmake: {}", e);
        std::process::exit(1);
    }
}
