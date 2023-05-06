// SPDX-FileCopyrightText: Copyright © 2023 {name}. All rights reserved.
// SPDX-License-Identifier: {license}

/// This is the main entry point for the {name} application.
fn main() {
    // Call the `run()` function from the `{name}` module.
    if let Err(err) = {name}::run() {
        eprintln!("Error running {name}: {}", err);
        std::process::exit(1);
    }
}
