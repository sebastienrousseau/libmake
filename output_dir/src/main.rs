// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// Copyright © 2024 test_lib. All rights reserved.
// SPDX-License-Identifier: MIT

/// This is the main entry point for the test_lib application.
fn main() {
    // Call the `run()` function from the `test_lib` module.
    if let Err(err) = test_lib::run() {
        eprintln!("Error running test_lib: {}", err);
        std::process::exit(1);
    }
}
