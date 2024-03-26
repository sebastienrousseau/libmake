// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// Copyright © 2024 {name}. All rights reserved.
// SPDX-License-Identifier: {license}

//! This crate contains the main entry point for the `{name}` application.

/// This is the main entry point for the `{name}` application.
fn main() {
    // Call the `run()` function from the `{name}` module.
    if let Err(err) = {name}::run() {
        eprintln!("Error running {name}: {}", err);
        std::process::exit(1);
    }
}
