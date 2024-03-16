// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! This is an example crate for `LibMake`.
//!
//! This crate provides various modules and examples for demonstrating the functionality
//! of `LibMake`. Each module focuses on a specific feature or functionality.
//!
//! Copyright © 2024 `LibMake`. All rights reserved.
//!
//! Dual-licensed under the terms of the Apache License, Version 2.0, or the MIT License,
//! at your option. See the 'LICENSE' file for details.

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
    // Run the example module `generate_from_args`.
    generate_from_args::main();

    // Run the example module `generate_from_config`.
    generate_from_config::main();

    // Run the example module `generate_from_csv`.
    generate_from_csv::main();

    // Run the example module `generate_from_json`.
    generate_from_json::main();

    // Run the example module `generate_from_toml`.
    generate_from_toml::main();

    // Run the example module `generate_from_yaml`.
    generate_from_yaml::main();

    // Run the example module `get_csv_field`.
    get_csv_field::main();

    // Run the example module `get_json_field`.
    get_json_field::main();

    // Run the example module `get_yaml_field`.
    get_yaml_field::main();
}
