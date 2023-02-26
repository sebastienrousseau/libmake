// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A code generator to reduce repetitive tasks and build high-quality Rust libraries
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/libmake/logo/logo-libmake.svg)](https://libmake.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/libmake.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/libmake)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.4-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/libmake)
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
//!
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/libmake/icons/ico-libmake.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/libmake/icons/ico-libmake.svg",
    html_root_url = "https://docs.rs/libmake"
)]
#![crate_name = "libmake"]
#![crate_type = "lib"]
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, TermLogger, TerminalMode};
use std::error::Error;

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
pub fn run() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    let log_config = Config::default();
    let file_logger = simplelog::WriteLogger::new(
        LevelFilter::Debug,
        log_config.clone(),
        std::fs::File::create("libmake.log")?,
    );
    let term_logger = TermLogger::new(
        LevelFilter::Info,
        log_config,
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );
    CombinedLogger::init(vec![term_logger, file_logger])?;

    // Process the ascii art
    ascii::generate_ascii_art("LibMake", "./resources/standard.flf");

    // Process the command-line arguments
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let matches = cli::build_cli()?;
    args::process_arguments(matches);

    Ok(())
}
