// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

// Module Declarations
// Each `mod` statement declares a module in the current crate.
// Modules help organize code into separate namespaces, each focusing on specific functionalities.

/// This is a module for the example `generate_from_args`.
mod generate_from_args;

/// This is a module for the example `generate_from_config`.
mod generate_from_config;

/// This is a module for the example `generate_from_csv`.
mod generate_from_csv;

/// This is a module for the example `generate_from_json`.
mod generate_from_json;

/// This is a module for the example `generate_from_toml`.
mod generate_from_toml;

/// This is a module for the example `generate_from_yaml`.
mod generate_from_yaml;

/// This is a module for the example `get_csv_field`.
mod get_csv_field;

/// This is a module for the example `get_json_field`.
mod get_json_field;

/// This is a module for the example `get_yaml_field`.
mod get_yaml_field;

// Main Function
// The `main` function serves as the entry point of the program.
// Here, it calls the `main` function of each module to execute their respective examples.

/// The main function that runs all the example modules.
fn main() {
    // Each line calls the `main()` function from a different module.
    // This executes the example associated with each module, demonstrating its functionality.
    generate_from_args::main();
    generate_from_config::main();
    generate_from_csv::main();
    generate_from_json::main();
    generate_from_toml::main();
    generate_from_yaml::main();
    get_csv_field::main();
    get_json_field::main();
    get_yaml_field::main();
}
