// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

//! Macros for the `libmake` crate.
//!
//! This module provides macros for generating templates from various configuration files
//! (JSON, YAML, CSV, INI, TOML), and for executing operations with logging.
//!
//! Each macro is documented with its purpose and usage, ensuring clear and maintainable code.
//! For detailed examples and error handling strategies, refer to the crate's documentation.

// Macro for generating file templates using a command-line interface.
#[macro_export]
/// Attempts to generate file templates from the given parameters.
///
/// # Arguments
///
/// * `$params` - Parameters for file generation, specified as `FileGenerationParams`.
///
/// # Returns
///
/// Returns a `Result<(), String>` indicating the success or failure of file generation.
/// If successful, returns `Ok(())`. If an error occurs, returns `Err` with an error message.
macro_rules! macro_generate_files {
    ($params:expr) => {{
        use $crate::generator::generate_files;
        match generate_files($params) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to generate files with parameters: {:?} - Error: {}", $params, e)),
        }
    }};
}

#[macro_export]
/// Attempts to generate file templates from a CSV file.
///
/// # Arguments
///
/// * `$csv_path` - The path to the CSV file as a string slice.
///
/// # Returns
///
/// Returns a `Result<(), String>` indicating the success or failure of file generation.
/// If successful, returns `Ok(())`. If an error occurs, returns `Err` with an error message.
macro_rules! macro_generate_from_csv {
    ($csv_path:expr) => {{
        use $crate::generators::csv::generate_from_csv;
        match generate_from_csv($csv_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to generate files from CSV at: {} - Error: {}", $csv_path, e)),
        }
    }};
}

#[macro_export]
/// Attempts to generate file templates from a INI file.
///
/// # Arguments
///
/// * `$ini_path` - The path to the INI file as a string slice.
///
/// # Returns
///
/// Returns a `Result<(), String>` indicating the success or failure of file generation.
/// If successful, returns `Ok(())`. If an error occurs, returns `Err` with an error message.
macro_rules! macro_generate_from_ini {
    ($ini_path:expr) => {{
        use $crate::generators::ini::generate_from_ini;
        match generate_from_ini($ini_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to generate files from INI at: {} - Error: {}", $ini_path, e)),
        }
    }};
}

#[macro_export]
/// Attempts to generate file templates from a JSON file.
///
/// # Arguments
///
/// * `$json_path` - The path to the JSON file as a string slice.
///
/// # Returns
///
/// Returns a `Result<(), String>` indicating the success or failure of file generation.
/// If successful, returns `Ok(())`. If an error occurs, returns `Err` with an error message.
macro_rules! macro_generate_from_json {
    ($json_path:expr) => {{
        use $crate::generators::json::generate_from_json;
        match generate_from_json($json_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to generate files from JSON at: {} - Error: {}", $json_path, e)),
        }
    }};
}

#[macro_export]
/// Attempts to generate file templates from a TOML file.
///
/// # Arguments
///
/// * `$toml_path` - The path to the TOML file as a string slice.
///
/// # Returns
///
/// Returns a `Result<(), String>` indicating the success or failure of file generation.
/// If successful, returns `Ok(())`. If an error occurs, returns `Err` with an error message.
macro_rules! macro_generate_from_toml {
    ($toml_path:expr) => {{
        use $crate::generators::toml::generate_from_toml;
        match generate_from_toml($toml_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to generate files from TOML at: {} - Error: {}", $toml_path, e)),
        }
    }};
}

#[macro_export]
/// Attempts to generate file templates from a YAML file.
///
/// # Arguments
///
/// * `$yaml_path` - The path to the YAML file as a string slice.
///
/// # Returns
///
/// Returns a `Result<(), String>` indicating the success or failure of file generation.
/// If successful, returns `Ok(())`. If an error occurs, returns `Err` with an error message.
macro_rules! macro_generate_from_yaml {
    ($yaml_path:expr) => {{
        use $crate::generators::yaml::generate_from_yaml;
        match generate_from_yaml($yaml_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to generate files from YAML at: {} - Error: {}", $yaml_path, e)),
        }
    }};
}

#[macro_export]
/// Attempts to generate file templates from command-line arguments.
///
/// # Arguments
///
/// * `$args` - Command-line arguments as a string slice.
///
/// # Returns
///
/// Returns a `Result<(), String>` indicating the success or failure of file generation.
/// If successful, returns `Ok(())`. If an error occurs, returns `Err` with an error message.
macro_rules! macro_generate_from_args {
    ($args:expr) => {{
        use $crate::generators::args::generate_from_args;
        match generate_from_args($args) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to generate files from arguments: {:?} - Error: {}", $args, e)),
        }
    }};
}

#[macro_export]
/// Attempts to generate file templates from a configuration file.
///
/// # Arguments
///
/// * `$path` - The path to the configuration file as a string slice.
/// * `$file_type` - The type of the configuration file (e.g., "json", "yaml").
///
/// # Returns
///
/// Returns a `Result<(), String>` indicating the success or failure of file generation.
/// If successful, returns `Ok(())`. If an error occurs, returns `Err` with an error message.
macro_rules! macro_generate_from_config {
    ($path:expr, $file_type:expr) => {{
        use $crate::generator::generate_from_config;
        match generate_from_config($path, $file_type) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to generate files from {} configuration at: {} - Error: {}", $file_type, $path, e)),
        }
    }};
}

#[macro_export]
/// Executes a shell command and logs the start, completion, and any errors.
///
/// # Parameters
///
/// * `$command` - The shell command to execute as a string slice.
/// * `$package` - The name of the package being operated on as a string slice.
/// * `$operation` - A description of the operation as a string slice.
/// * `$start_message` - The message to log at the start of the operation as a string slice.
/// * `$complete_message` - The message to log upon successful completion as a string slice.
/// * `$error_message` - The message to log in case of an error as a string slice.
///
/// # Returns
///
/// Returns a `Result<(), anyhow::Error>` to indicate the success or failure of the command execution.
macro_rules! macro_execute_and_log {
    ($command:expr, $package:expr, $operation:expr, $start_message:expr, $complete_message:expr, $error_message:expr) => {{
        use anyhow::{Context, Result as AnyResult};
        // Using a fictional logging framework for demonstration purposes. You'll need to replace
        // crate::log with your actual logging implementation.
        $crate::log::info!("Starting operation: {}", $start_message);

        std::process::Command::new("sh")
            .arg("-c")
            .arg($command)
            .output()
            .map(|output| {
                if output.status.success() {
                    $crate::log::info!("Operation completed successfully: {}", $complete_message);
                    Ok(())
                } else {
                    $crate::log::error!("Operation failed: {} - Error: {}", $error_message, String::from_utf8_lossy(&output.stderr));
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "Command execution failed").into())
                }
            })
            .with_context(|| {
                format!(
                    "Failed to execute '{}' for {} on package '{}'",
                    stringify!($command),
                    $operation,
                    $package
                )
            })
    }};
}

/// Extracts a parameter from a `Matches` object.
///
/// This macro takes two arguments: `$matches` and `$name`. It attempts to retrieve the value
/// associated with `$name` from the `$matches` object. If the value is found and is of type `String`,
/// it returns a `Some` variant containing a cloned copy of the value. Otherwise, it returns `None`.
///
/// # Arguments
///
/// * `$matches` - A `Matches` object that contains the parameter values.
/// * `$name` - The name of the parameter to extract.
///
/// # Returns
///
/// A `Option<String>` containing the extracted parameter value, or `None` if the parameter is not found
/// or is not of type `String`.
#[macro_export]
macro_rules! extract_param {
    ($matches:expr, $name:expr) => {
        $matches.get_one::<String>($name).map(|s| s.to_owned())
    };
}
