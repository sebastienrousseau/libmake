use csv::ReaderBuilder;
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
pub fn get_csv_field(file_path: Option<&str>, field_name: &str) -> String {
    if let Some(file_path) = file_path {
        let file = File::open(Path::new(file_path)).unwrap();
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

        // Read the header row
        let headers = reader.headers().unwrap();

        // Find the index of the desired field in the header row
        let index = headers.iter().position(|h| h == field_name).unwrap();

        // Find the value of the desired field in each record
        for result in reader.records() {
            let record = result.unwrap();
            if let Some(value) = record.get(index) {
                return value.to_string();
            }
        }
        "".to_string()
    } else {
        "".to_string()
    }
}

/// Reads a JSON file at the given file path and returns the value of
/// the given field.
///
/// # Arguments
///
/// * `file_path` - An optional string slice that holds the file path of the JSON file to read.
/// * `field_name` - A string slice that holds the name of the field to retrieve.
///
pub fn get_json_field(file_path: Option<&str>, field_name: &str) -> String {
    if let Some(file_path) = file_path {
        let file = File::open(Path::new(file_path)).unwrap();
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();
        json[field_name].to_string()
    } else {
        "".to_string()
    }
}

/// Reads a YAML file at the given file path and returns the value of
/// the given field.
///
pub fn get_yaml_field(file_path: Option<&str>, field_name: &str) -> String {
    if let Some(file_path) = file_path {
        let file = File::open(Path::new(file_path)).unwrap();
        let yaml: serde_yaml::Value = serde_yaml::from_reader(file).unwrap();
        let field_value = &yaml[field_name];
        let field_value_str = serde_yaml::to_string(&field_value).unwrap();
        field_value_str.trim().to_string()
    } else {
        "".to_string()
    }
}
