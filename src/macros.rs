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
