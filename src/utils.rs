// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

use serde_json::Value;
use serde_yaml::from_reader;
use serde_yaml::{from_str, to_string, Value as YamlValue};
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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

/// Retrieves a specific field's value from a JSON file.
///
/// # Arguments
///
/// * `file_path` - An optional reference to the path of the JSON file.
/// * `field_name` - The name of the field to retrieve the value from.
///
/// # Returns
///
/// Returns a `Result<String, Box<dyn std::error::Error>>` containing the value of the specified field, or an error if one occurs.
///
pub fn get_json_field(
    file_path: Option<&str>,
    field_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    file_path.map_or_else(
        || Ok(String::new()),
        |file_path| {
            let current_dir = env::current_dir()?;
            let file_path = Path::new(&current_dir).join(file_path);
            read_file(&file_path, |file| {
                let json: Value = serde_json::from_reader(file)?;
                let json_value = json[field_name].clone();
                let field_value = match json_value.as_str() {
                    Some(s) => s.to_string(),
                    None => {
                        let binding = json_value.to_string();
                        let trimmed_binding =
                            binding.trim_matches('"').to_string();
                        trimmed_binding
                    }
                };
                Ok(field_value)
            })
        },
    )
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
/// Returns a `Result<String, Box<dyn std::error::Error>>` containing the value of the specified field, or an error if one occurs.
///
pub fn get_yaml_field(
    file_path: Option<&str>,
    field_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    file_path.map_or_else(
        || Ok(String::new()),
        |file_path| {
            let current_dir = env::current_dir()?;
            let file_path = Path::new(&current_dir).join(file_path);
            read_file(&file_path, |file| {
                let yaml: serde_yaml::Value = from_reader(file)?;
                println!("YAML Value: {:?}", yaml);
                let field_value = &yaml[field_name];
                println!("Field Value: {:?}", field_value);
                let field_value_str = to_string(&field_value)?;
                Ok(field_value_str.trim().to_string())
            })
        },
    )
}

/// Retrieves a specific field's value from a configuration file.
///
/// # Arguments
///
/// * `file_path` - An optional reference to the path of the configuration file.
/// * `file_format` - The format of the configuration file ("json" or "yaml").
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

    // Ensure file_format is provided and is either 'json' or 'yaml'
    let format = file_format.ok_or("File format is not provided")?;
    match format {
        "json" => {
            // Read JSON file and extract field value
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            let json: Value = serde_json::from_reader(reader)?;
            let field_value = json.get(field_name).ok_or("Field not found in JSON")?;

            // Handle JSON string values to remove surrounding quotes
            match field_value.as_str() {
                Some(s) => Ok(s.to_string()),
                None => {
                    let s = field_value.to_string();
                    Ok(s.trim_matches('"').to_string())
                }
            }
        },
        "yaml" => {
            let yaml_str = fs::read_to_string(file_path)?;
            let yaml_value: YamlValue = from_str(&yaml_str)?;
            let field_value = yaml_value.get(field_name).ok_or("Field not found in YAML")?;

            // Convert the field_value to a string
            let field_value_str = match field_value {
                YamlValue::String(s) => s.clone(),
                _ => to_string(field_value).unwrap(),
            };

            Ok(field_value_str)
        }
        _ => Err(format!("Unsupported file format: {}. Supported formats are 'json' and 'yaml'.", format).into())
    }
}
