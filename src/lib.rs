// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A code generator to reduce repetitive tasks and build high-quality Rust libraries.
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
//! The LibMake (libmake) library is a Rust library generator that helps
//! create high-quality Rust libraries quickly and easily.
//!
//! The library is designed to be used as a command-line tool. It is
//! available on [crates.io](https://crates.io/crates/libmake) and
//! [lib.rs](https://lib.rs/crates/libmake).
//!
//! ## Features
//!
//! - Generates a new Rust library manually via the command line.
//! - Generates a new Rust library from a CSV file.
//! - Generates a new Rust library from a JSON file.
//! - Generates a new Rust library from a YAML file.
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
