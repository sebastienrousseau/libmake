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
        println!("Value: {}", field_value);
    }
    println!("Values: {:?}", values);
    Some(values)
}

/// Reads a JSON file at the given file path and returns the value of
/// the given field.
///
/// # Arguments
///
/// * `file_path` - An optional string slice that holds the file path of the JSON file to read.
/// * `field_name` - A string slice that holds the name of the field to retrieve.
///
pub fn get_json_field(
    file_path: Option<&str>,
    field_name: &str,
) -> String {
    if let Some(file_path) = file_path {
        let file = File::open(Path::new(file_path)).unwrap();
        let json: serde_json::Value =
            serde_json::from_reader(file).unwrap();
        json[field_name].to_string()
    } else {
        "".to_string()
    }
}

/// Reads a YAML file at the given file path and returns the value of
/// the given field.
///
pub fn get_yaml_field(
    file_path: Option<&str>,
    field_name: &str,
) -> String {
    if let Some(file_path) = file_path {
        let file = File::open(Path::new(file_path)).unwrap();
        let yaml: serde_yaml::Value =
            serde_yaml::from_reader(file).unwrap();
        let field_value = &yaml[field_name];
        let field_value_str =
            serde_yaml::to_string(&field_value).unwrap();
        field_value_str.trim().to_string()
    } else {
        "".to_string()
    }
}

/// Reads as Configuration file at the given file path, file type and
/// field name and returns the value of the given field.
pub fn get_config_field(
    file_path: Option<&str>,
    file_type: Option<&str>,
    field_name: &str,
) -> String {
    if let Some(file_type) = file_type {
        match file_type {
            "json" => get_json_field(file_path, field_name),
            "yaml" => get_yaml_field(file_path, field_name),
            _ => "".to_string(),
        }
    } else {
        "".to_string()
    }
}
