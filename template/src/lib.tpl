// Copyright © {{YEAR}} {{PROJECTNAME}}. All rights reserved.
// SPDX-License-Identifier: {{SPDX_LICENSE}}
//!
//! # {{DESCRIPTION}}
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-{{LIBNAME}}.svg)](https://minifunctions.com)
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/{{LIBNAME}}.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/{{LIBNAME}})
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v{{VERSION}}-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/{{LIBNAME}})
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)]({{GITHUBURL}}/{{LIBNAME}})
//! [![License](https://img.shields.io/crates/l/{{LIBNAME}}.svg?style=for-the-badge&color=007EC6&labelColor=03589B)]({{LICENSEURL}})
//!
//! {{OVERVIEW}}
//!
//! ## Features
//!
//! - Serialization and deserialization of data structures to JSON format
//! - ...
//! - ...
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! {{LIBNAME}} = "{{VERSION}}"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//!
//! Use the following code to serialize a data structure to JSON format:
//!
//! ```rust
//! use {{LIBNAME}}::{{LIBNAME}};
//! use serde_json;
//!
//! let my_data = {{LIBNAME}}::new();
//! let json = serde_json::to_string(&my_data).unwrap();
//! ```
//!
//! Use the following code to deserialize a JSON string into a data structure:
//!
//! ```rust
//! use {{LIBNAME}}::{{LIBNAME}};
//! use serde_json;
//!
//! let json = r#"{"foo": "bar"}"#;
//! let my_data: {{LIBNAME}} = serde_json::from_str(json).unwrap();
//! ```
//!
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![doc(
    html_favicon_url = "{{FAVICONURL}}",
    html_logo_url = "{{LOGOURL}}",
    html_root_url = "https://docs.rs/{{LIBNAME}}"
)]
#![crate_name = "{{LIBNAME}}"]
#![crate_type = "lib"]

use serde::{Deserialize, Serialize};
pub use self::error::Error;
mod error;


#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct {{LIBNAME}} {
    // Add any data fields needed here
}

impl {{LIBNAME}} {
    pub fn new() -> Self {
        Self {
            // Initialize any data fields here
        }
    }
}

impl Default for {{LIBNAME}} {
    fn default() -> Self {
        Self::new()
    }
}
