// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! # `LibMake`
//!
//! A code generator to reduce repetitive tasks and build high-quality Rust libraries.
//!
//! *Part of the [Mini Functions][0] family of libraries.*
//!
//! [![Rust](https://kura.pro/libmake/images/banners/banner-libmake.svg)](https://libmake.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/libmake.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/libmake)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.2.5-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/libmake)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/libmake)
//! [![License](https://img.shields.io/crates/l/libmake.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! `LibMake` is a tool designed to quickly help create high-quality
//! Rust libraries by generating a set of pre-filled and pre-defined
//! templated files. This opinionated boilerplate scaffolding tool aims
//! to greatly reduce development time and minimize repetitive tasks,
//! allowing you to focus on your business logic while enforcing
//! standards, best practices, consistency, and providing style guides
//! for your library.
//!
//! With `LibMake`, you can easily generate a new Rust library code base
//! structure with all the necessary files, layouts, build
//! configurations, code, tests, benchmarks, documentation, and much
//! more in a matter of seconds.
//!
//! The library is designed to be used as a command-line tool. It is
//! available on [Crates.io](https://crates.io/crates/libmake) and
//! [Lib.rs](https://lib.rs/crates/libmake).
//!
//! ## Features
//!
//! `LibMake` offers the following features and benefits:
//!
//! - Create your Rust library with ease using the command-line
//!   interface or by providing a configuration file in CSV, JSON, or
//!   YAML format.
//! - Rapidly generate new library projects with a pre-defined structure
//!   and boilerplate code that you can customize with your own template.
//! - Automatically generate basic functions, methods, and macros to get
//!   you started with your Rust library.
//! - Enforce best practices and standards with starter documentation,
//!   test suites, and benchmark suites that are designed to help you
//!   get up and running quickly.
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde
//!
//! [`serde`]: https://github.com/serde-rs/serde
//! [0]: https://minifunctions.com/libmake "Mini Functions"
//!
#![allow(clippy::must_use_candidate)]
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(rustc::existing_doc_keyword)]
#![doc(
    html_favicon_url = "https://kura.pro/libmake/images/favicon.ico",
    html_logo_url = "https://kura.pro/libmake/images/logos/libmake.svg",
    html_root_url = "https://docs.rs/libmake"
)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(unreachable_pub)]
#![forbid(unsafe_code)]
#![crate_name = "libmake"]
#![crate_type = "lib"]

// Import necessary dependencies
use crate::args::process_arguments;
use crate::cli::build;
use dtt::DateTime;
use rlg::{log_format::LogFormat, log_level::LogLevel, macro_log};
use std::{error::Error, fs::File, io::Write};
use crate::utilities::uuid::generate_unique_string;

/// The `args` module contains functions for processing command-line
/// arguments.
pub mod args;
/// The `generators` module contains functions for generating code.
pub mod generators;
/// The `cli` module contains functions for processing command-line
/// input.
pub mod cli;
/// The `generator` module contains functions for generating the new
/// library.
pub mod generator;
/// The `interface` module contains functions for displaying the
/// interface.
pub mod interface;
/// The `macros` module contains functions for generating macros.
pub mod macros;
/// The `models` module contains the models for the library.
pub mod models;
/// The `utils` module contains a function for reading a CSV file at the
/// given file path and returns the value of the given field.
pub mod utils;

/// The `utilities` module contains utility functions for the library.
pub mod utilities;

/// Initializes the logger with a file logger and a terminal logger and processes
/// command-line arguments to generate the new library.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If there is a problem generating ASCII art for the tool's CLI.
/// - If an error occurs while building the command-line interface or processing arguments.
/// - Any other errors that may arise from operations performed within the function.
///
pub fn run() -> Result<(), Box<dyn Error>> {
    let date = DateTime::new();
    let iso = date.iso_8601;

    // Open the log file for appending
    let mut log_file = File::create("./libmake.log")?;

    // Generate ASCII art for the tool's CLI
    let log = macro_log!(
        &generate_unique_string(),
        &iso,
        &LogLevel::INFO,
        "deps",
        "ASCII art generation event started.",
        &LogFormat::CLF
    );
    // Write the log to both the console and the file
    writeln!(log_file, "{}", log)?;

    // Printing the ASCII art to the console
    println!("{}", macro_ascii!("LibMake"));

    let log = macro_log!(
        &generate_unique_string(),
        &iso,
        &LogLevel::INFO,
        "deps",
        "ASCII art generation event completed.",
        &LogFormat::CLF
    );
    // Write the log to both the console and the file
    writeln!(log_file, "{}", log)?;

    // Build the command-line interface and process the arguments
    let matches = build()?;
    process_arguments(&matches)?;

    // Check the number of arguments, provide a welcome message if no arguments were passed
    macro_log!(
        &generate_unique_string(),
        &iso,
        &LogLevel::INFO,
        "cli",
        "Welcome to LibMake! 👋\n\nLet's get started! Please, run `libmake --help` for more information.",
        &LogFormat::CLF
    );
    eprintln!(
        "\n\nWelcome to LibMake! 👋\n\nLet's get started! Please, run `libmake --help` for more information.\n"
    );

    Ok(())
}
