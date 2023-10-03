// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//!
//! A code generator to reduce repetitive tasks and build high-quality Rust
//! libraries.
//!
//! *Part of the [Mini Functions][0] family of libraries.*
//!
//! [![Rust](https://kura.pro/libmake/images/banners/banner-libmake.svg)](https://libmake.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/libmake.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/libmake)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.1.9-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/libmake)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/libmake)
//! [![License](https://img.shields.io/crates/l/libmake.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! `LibMake` is a tool designed to quickly help creating high-quality
//! Rust libraries by generating a set of pre-filled and pre-defined
//! templated files. This opinionated boilerplate scaffolding tool aims
//! to greatly reduces development time and minimizes repetitive tasks,
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
//! - Create your Rust library with ease using the command line
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
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(rustc::existing_doc_keyword)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(unreachable_pub)]
#![forbid(unsafe_code)]
#![doc(
    html_favicon_url = "https://kura.pro/libmake/images/favicon.ico",
    html_logo_url = "https://kura.pro/libmake/images/logos/libmake.svg",
    html_root_url = "https://docs.rs/libmake"
)]
#![crate_name = "libmake"]
#![crate_type = "lib"]

/// The `args` module contains functions for processing command-line
/// arguments.
pub mod args;
/// The `ascii` module contains functions for generating ASCII art.
pub mod ascii;
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
/// The `utils` module contains a function for reading a CSV file at the
/// given file path and returns the value of the given field.
pub mod utils;

/// Initializes the logger with a file logger and a terminal logger.
///
/// # Examples
///
/// ```
/// use libmake::run;
/// run();
/// ```
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Process the ascii art
    ascii::generate_ascii_art("LibMake");

    // Process the command-line arguments
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let matches = cli::build_cli()?;
    args::process_arguments(&matches);

    // Print the welcome message if no arguments were passed
    if std::env::args().len() == 1 {
        eprintln!(
            "\n\nWelcome to LibMake! 👋\n\nLet's get started! Please, run `libmake --help` for more information.\n"
        );
    }
    Ok(())
}
