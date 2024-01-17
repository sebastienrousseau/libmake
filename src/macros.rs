// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! Macros for the `libmake` crate.
//!
//! This module provides macros for asserting the creation of directories,
//! generating templates from JSON, YAML, and CSV files, and custom logging functionality.

// Macro for creating a new directory at the specified path.
#[macro_export]
/// Asserts that a directory is created at the given path.
///
/// # Arguments
///
/// * `$path` - A string literal specifying the path where the directory will be created.
///
/// # Panics
///
/// Panics if the directory cannot be created.
macro_rules! assert_create_directory {
    ($path:expr) => {
        use std::fs::{self, create_dir_all};
        use std::path::Path;
        assert!(
            create_dir_all(Path::new($path)).is_ok(),
            "Failed to create directory at: {}",
            $path
        );
    };
}

// Macro for generating file templates using a command-line interface.
#[macro_export]
/// Asserts that file templates are generated from the given parameters.
///
/// # Arguments
///
/// * `$params` - Parameters for file generation.
///
/// # Panics
///
/// Panics if the files cannot be generated.
macro_rules! assert_generate_files {
    ($params:expr) => {
        assert!(
            generate_files($params).is_ok(),
            "Failed to generate files with parameters: {:?}",
            $params
        );
    };
}

// Macro for generating file templates from a CSV file.
#[macro_export]
/// Asserts that file templates are generated from a CSV file.
///
/// # Arguments
///
/// * `$csv_path` - The path to the CSV file.
///
/// # Panics
///
/// Panics if the files cannot be generated from the CSV.
macro_rules! assert_generate_from_csv {
    ($csv_path:expr) => {
        assert!(
            generate_from_csv($csv_path).is_ok(),
            "Failed to generate files from CSV at: {}",
            $csv_path
        );
    };
}

// Macro for generating file templates from a JSON file.
#[macro_export]
/// Asserts that file templates are generated from a JSON file.
///
/// # Arguments
///
/// * `$json_path` - The path to the JSON file.
///
/// # Panics
///
/// Panics if the files cannot be generated from the JSON.
macro_rules! assert_generate_from_json {
    ($json_path:expr) => {
        assert!(
            generate_from_json($json_path).is_ok(),
            "Failed to generate files from JSON at: {}",
            $json_path
        );
    };
}

// Macro for generating file templates from a YAML file.
#[macro_export]
/// Asserts that file templates are generated from a YAML file.
///
/// # Arguments
///
/// * `$yaml_path` - The path to the YAML file.
///
/// # Panics
///
/// Panics if the files cannot be generated from the YAML.
macro_rules! assert_generate_from_yaml {
    ($yaml_path:expr) => {
        assert!(
            generate_from_yaml($yaml_path).is_ok(),
            "Failed to generate files from YAML at: {}",
            $yaml_path
        );
    };
}

// Macro for generating file templates from a configuration file.
#[macro_export]
/// Asserts that file templates are generated from a configuration file.
///
/// # Arguments
///
/// * `$path` - The path to the configuration file.
/// * `$file_type` - The type of the configuration file: `json`, `yaml`, `yml`, or `csv`.
///
/// # Panics
///
/// Panics if the files cannot be generated from the configuration file.
macro_rules! assert_generate_from_config {
    ($path:expr, $file_type:expr) => {
        assert!(
            generate_from_config($path, $file_type).is_ok(),
            "Failed to generate files from {} configuration at: {}",
            $file_type,
            $path
        );
    };
}
// Macro for logging information with various log levels and formats.
#[macro_export]
/// Logs information with the specified level, component, and format.
///
/// # Parameters
///
/// * `$level` - The log level for the message.
/// * `$component` - The component where the log message originates.
/// * `$description` - A description for the log message.
/// * `$format` - The format for the log message.
///
/// # Returns
///
/// This macro returns the created `Log` instance.
macro_rules! macro_log_info {
    ($level:expr, $component:expr, $description:expr, $format:expr) => {{
        use dtt::DateTime;
        use vrd::Random;
        use $crate::loggers::{Log, LogFormat, LogLevel};

        // Get the current date and time in ISO 8601 format.
        let date = DateTime::new();
        let iso = date.iso_8601;

        // Create a new random number generator
        let mut rng = Random::default();
        let session_id = rng.rand().to_string();

        let log = Log::new(
            &session_id,
            &iso,
            $level,
            $component,
            $description,
            $format,
        );
        let _ = log.log();
        log // Return the Log instance
    }};
}
// Macro for executing a shell command and logging the operation.
#[macro_export]
/// Executes a shell command and logs the start, completion, and any errors.
///
/// # Parameters
///
/// * `$command` - The shell command to execute.
/// * `$package` - The name of the package being operated on.
/// * `$operation` - A description of the operation.
/// * `$start_message` - The message to log at the start of the operation.
/// * `$complete_message` - The message to log upon successful completion.
/// * `$error_message` - The message to log in case of an error.
///
/// # Returns
///
/// Returns a `Result<(), anyhow::Error>` to indicate the success or failure of the command execution.
macro_rules! macro_execute_and_log {
    ($command:expr, $package:expr, $operation:expr, $start_message:expr, $complete_message:expr, $error_message:expr) => {{
        use anyhow::{Context, Result as AnyResult};
        use $crate::loggers::{LogFormat, LogLevel};
        use $crate::macro_log_info;

        macro_log_info!(
            LogLevel::INFO,
            $operation,
            $start_message,
            LogFormat::CLF
        );

        $command
            .run()
            .map(|_| ())
            .map_err(|err| {
                macro_log_info!(
                    LogLevel::ERROR,
                    $operation,
                    $error_message,
                    LogFormat::CLF
                );
                err
            })
            .with_context(|| {
                format!(
                    "Failed to execute '{}' for {} on package '{}'",
                    stringify!($command),
                    $operation,
                    $package
                )
            })?;

        macro_log_info!(
            LogLevel::INFO,
            $operation,
            $complete_message,
            LogFormat::CLF
        );
        Ok(())
    }};
}
