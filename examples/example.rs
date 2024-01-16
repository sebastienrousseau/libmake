// Copyright © 2024 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

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

/// The main function that runs all the example modules.
fn main() {
    // Run the `main()` function from each module to execute the examples.
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
