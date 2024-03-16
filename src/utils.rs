// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

use std::env;
use std::fs::File;
use std::path::Path;

/// Reads a CSV file at the given file path and returns the value of
/// the given field.
///
/// # Arguments
///
/// * `file_path` - An optional string slice that holds the file path of the CSV file to read.
/// * `field_name` - A string slice that holds the name of the field to retrieve.
///
pub fn get_csv_field(
    file: Option<&str>,
    field_index: usize,
) -> Option<Vec<String>> {
    let current_dir = env::current_dir().ok()?;
    let file_path = Path::new(&current_dir).join(file?);
    let file = File::open(file_path).ok()?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut values = Vec::new();
    for result in rdr.records() {
        let record = result.ok()?;
        let field_value = record.get(field_index).unwrap_or("");
        values.push(field_value.to_string());
        println!("Value: {field_value}");
    }
    println!("Values: {values:?}");
    Some(values)
}

/// Retrieves a specific field's value from a JSON file.
///
/// # Arguments
///
/// * `file_path` - An optional reference to the path of the JSON file.
/// * `field_name` - The name of the field to retrieve the value from.
///
/// # Returns
///
/// Returns a `String` containing the value of the specified field.
///
/// # Panics
///
/// Panics if the file specified by `file_path` cannot be opened or if the content of the file
/// is not valid JSON. This can happen due to various reasons like the file not existing,
/// being unreadable, or JSON parsing failures.
///
pub fn get_json_field(
    file_path: Option<&str>,
    field_name: &str,
) -> String {
    file_path.map_or_else(String::new, |file_path| {
        let file = File::open(Path::new(file_path)).unwrap();
        let json: serde_json::Value =
            serde_json::from_reader(file).unwrap();
        json[field_name].to_string()
    })
}

/// Retrieves a specific field's value from a YAML file.
///
/// # Arguments
///
/// * `file_path` - An optional reference to the path of the YAML file.
/// * `field_name` - The name of the field to retrieve the value from.
///
/// # Returns
///
/// Returns a `String` containing the value of the specified field.
///
/// # Panics
///
/// Panics if:
///
/// - The file specified by `file_path` cannot be opened.
/// - The file content is not valid YAML or does not contain the specified `field_name`.
/// - Fails to serialize the YAML content to a string.
///
pub fn get_yaml_field(
    file_path: Option<&str>,
    field_name: &str,
) -> String {
    file_path.map_or_else(String::new, |file_path| {
        let file = match File::open(Path::new(file_path)) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file: {e}");
                return String::new(); // Return a default value on error
            }
        };

        let yaml: serde_yaml::Value =
            match serde_yaml::from_reader(file) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error reading YAML: {e}");
                    return String::new(); // or handle the error as appropriate
                }
            };
        let field_value = &yaml[field_name];
        let field_value_str =
            serde_yaml::to_string(&field_value).unwrap();
        field_value_str.trim().to_string()
    })
}

/// Retrieves a specific field's value from a JSON file.
///
/// # Arguments
///
/// * `file_path` - An optional reference to the path of the JSON file.
/// * `field_name` - The name of the field to retrieve the value from.
///
/// # Returns
///
/// Returns a `String` containing the value of the specified field.
///
/// # Panics
///
/// Panics if:
///
/// - The file specified by `file_path` cannot be opened.
/// - The file content is not valid JSON or does not contain the specified `field_name`.
///
pub fn get_config_field(
    file_path: Option<&str>,
    file_type: Option<&str>,
    field_name: &str,
) -> String {
    file_type.map_or_else(
        String::new, // Provide a default value for the None case
        |file_type| match file_type {
            // Transform the value in the Some case
            "json" => get_json_field(file_path, field_name),
            "yaml" => get_yaml_field(file_path, field_name),
            _ => String::new(),
        },
    )
}
