// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

use std::{env, fs::File, path::Path};
use crate::macro_get_field;


/// Reads a file and deserializes its content using the specified deserializer function.
///
/// # Arguments
///
/// * `file_path` - The path of the file to read.
/// * `deserializer` - A function that takes a `File` and returns a deserialized value of type `T`.
///
/// # Returns
///
/// Returns a `Result<T, Box<dyn std::error::Error>>` containing the deserialized value, or an error if one occurs.
///
fn read_file<T, F>(
    file_path: &Path,
    deserializer: F,
) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnOnce(File) -> Result<T, Box<dyn std::error::Error>>,
{
    let file = File::open(file_path)?;
    deserializer(file)
}

/// Reads a CSV file at the given file path and returns the value of
/// the given field.
///
/// # Arguments
///
/// * `file_path` - An optional string slice that holds the file path of the CSV file to read.
/// * `field_index` - The index of the field to retrieve.
///
pub fn get_csv_field(
    file_path: Option<&str>,
    field_index: usize,
) -> Option<Vec<String>> {
    file_path.and_then(|file_path| {
        let current_dir = env::current_dir().ok()?;
        let file_path = Path::new(&current_dir).join(file_path);
        let file = File::open(file_path).ok()?;
        let mut rdr = csv::Reader::from_reader(file);

        let mut values = Vec::new();
        for result in rdr.records() {
            let record = result.ok()?;
            if let Some(field_value) = record.get(field_index) {
                values.push(field_value.to_string());
            } else {
                // Field index is out of range
                return None;
            }
        }
        if values.is_empty() {
            None
        } else {
            Some(values)
        }
    })
}

macro_get_field!(get_ini_field, serde_ini::from_read);
macro_get_field!(get_json_field, serde_json::from_reader);
macro_get_field!(get_yaml_field, serde_yaml::from_reader);

/// Retrieves a specific field's value from a configuration file.
///
/// # Arguments
///
/// * `file_path` - An optional reference to the path of the configuration file.
/// * `file_format` - The format of the configuration file ("json", "yaml", or "ini").
/// * `field_name` - The name of the field to retrieve the value from.
///
/// # Returns
///
/// Returns a `Result<String, Box<dyn std::error::Error>>` containing the value of the specified field, or an error if one occurs.
///
pub fn get_config_field(
    file_path: Option<&str>,
    file_format: Option<&str>,
    field_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Ensure file_path is provided
    let file_path = file_path.ok_or("File path is not provided")?;

    // Ensure file_format is provided and is either 'json', 'yaml', or 'ini'
    let format = file_format.ok_or("File format is not provided")?;
    match format {
        "ini" => get_ini_field(Some(file_path), field_name),
        "json" => get_json_field(Some(file_path), field_name),
        "yaml" => get_yaml_field(Some(file_path), field_name),
        _ => Err(format!(
            "Unsupported file format: {}. Supported formats are 'json', 'yaml', and 'ini'.",
            format
        )
        .into()),
    }
}
