// SPDX-FileCopyrightText: Copyright © 2023 {name}. All rights reserved.
// SPDX-License-Identifier: {license}
//!
//! # `{name}` 🦀
//!
//! [![{name}](https://via.placeholder.com/1500x500.png/000000/FFFFFF?text={name})]({website} "{name} - {description}")
//!
//! {description}
//!
//! [![Crates.io](https://img.shields.io/crates/v/{name}.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/{name} "Crates.io")
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v{version}-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/{name} "Lib.rs")
//! [![License](https://img.shields.io/crates/l/{name}.svg?style=for-the-badge&color=007EC6&labelColor=03589B)]({license}  "{license}")
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org "Rust")
//!
//! ## Overview
//!
//! {description}
//!
//! ## Features
//!
//! - ...
//! - ...
//! - ...
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! {name} = "{version}"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//!
//! ## Examples
//!
//! Check out the examples folder for helpful snippets of code that
//! demonstrate how to use the `{name}` library. You can also check out
//! the [documentation](https://docs.rs/{name}) for more information on
//! how to use the library.
//!
//! ```rust
//!    use {name}::{name};
//!
//! ```
//!
//! ## License
//!
//! The project is licensed under the terms of the {license} license.
//!
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "",
    html_logo_url = "",
    html_root_url = "https://docs.rs/{name}"
)]
#![crate_name = "{name}"]
#![crate_type = "lib"]

/// The `loggers` module contains the loggers for the library.
pub mod loggers;

/// The `macros` module contains functions for generating macros.
pub mod macros;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[non_exhaustive]
#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]

#[allow(non_camel_case_types)]
/// {name} is a data structure that ...
pub struct {name} {
    // Add any data fields needed here
}

/// This is the main entry point for the {name} library.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Add your code here
    let name = "{name}";
    println!("Hello, {}!", { name }.to_uppercase());
    Ok(())
}


impl {name} {
    /// Creates a new instance of {name}
    pub fn new() -> Self {
        Self {
            // Initialize any data fields here
        }
    }
}

impl Default for {name} {
    /// Creates a new instance of {name} with default values
    fn default() -> Self {
        Self::new()
    }
}