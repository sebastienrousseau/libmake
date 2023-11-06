// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Macros for the `libmake` crate.
//!
//! This module bundles all macros used across the `libmake` crate.
//! These include macros for asserting the creation of directories,
//! generating templates from JSON, YAML, and CSV files.
//!

#[macro_export]
/// A macro for creating a new directory.
///
/// This will create a new directory at the given path. If the directory
/// already exists, it will return an error.
///
macro_rules! assert_create_directory {
    ($path:expr) => {
        assert!(create_directory(Path::new($path)).is_ok());
    };
}
#[macro_export]
/// A macro for generating a new set of file templates using the command
/// line interface.
macro_rules! assert_generate_files {
    ($params:expr) => {
        assert!(generate_files($params).is_ok());
    };
}
#[macro_export]
/// A macro for generating a new set of file templates using a CSV file.
macro_rules! assert_generate_from_csv {
    ($csv_path:expr) => {
        assert!(generate_from_csv($csv_path).is_ok());
    };
}
#[macro_export]
/// A macro for generating a new set of file templates using a JSON file.
macro_rules! assert_generate_from_json {
    ($path:expr) => {
        assert!(generate_from_json($path).is_ok());
    };
}
#[macro_export]
/// A macro for generating a new set of file templates using a YAML file.
macro_rules! assert_generate_from_yaml {
    ($path:expr) => {
        assert!(generate_from_yaml($path).is_ok());
    };
}
#[macro_export]
/// A macro for generating a new set of file templates using a
/// configuration file. This will require a path to the configuration
/// file and the file type. The file type can be either `json`, `yaml`,
/// `yml` or `csv`.
macro_rules! assert_generate_from_config {
    ($path:expr, $file_type:expr) => {
        assert!(generate_from_config($path, $file_type).is_ok());
    };
}
/// Custom logging macro for various log levels and formats.
///
/// # Parameters
///
/// * `$level`: The log level of the message.
/// * `$component`: The component where the log is coming from.
/// * `$description`: A description of the log message.
/// * `$format`: The format of the log message.
///
#[macro_export]
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
/// Macros related to executing shell commands.
///
/// Executes a shell command, logs the start and completion of the operation, and handles any errors that occur.
///
/// # Parameters
///
/// * `$command`: The shell command to execute.
/// * `$package`: The name of the package the command is being run on.
/// * `$operation`: A description of the operation being performed.
/// * `$start_message`: The log message to be displayed at the start of the operation.
/// * `$complete_message`: The log message to be displayed upon successful completion of the operation.
/// * `$error_message`: The log message to be displayed in case of an error.
///
/// # Returns
///
/// Returns a `Result<(), anyhow::Error>` indicating the success or failure of the operation.
///
#[macro_export]
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
